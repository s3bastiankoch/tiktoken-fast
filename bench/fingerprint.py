"""Records what a benchmark run ran on, in <run dir>/machine.json.

    python bench/fingerprint.py start <run dir> <workload dir> <core>
    python bench/fingerprint.py end <run dir>

start records the CPU, the cloud instance type, the kernel, the CPU frequency
governor, every toolchain and library version, the commit, and the SHA-256 of
every workload file, so two runs can be shown to have timed the same inputs.
end adds the share of CPU time the hypervisor took from the machine while it
ran (steal), which would show a noisy neighbour.
"""
import datetime
import glob
import hashlib
import json
import os
import platform
import subprocess
import sys
import urllib.request

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.dirname(HERE)


def sh(*cmd, cwd=None):
    try:
        return subprocess.run(cmd, capture_output=True, text=True, cwd=cwd, timeout=60).stdout.strip()
    except Exception:
        return ""


def cpu_model():
    for line in open("/proc/cpuinfo") if os.path.exists("/proc/cpuinfo") else []:
        if line.startswith("model name"):
            return line.split(":", 1)[1].strip()
    lscpu = sh("lscpu")
    for line in lscpu.splitlines():
        if line.startswith("Model name"):
            return line.split(":", 1)[1].strip()
    return sh("sysctl", "-n", "machdep.cpu.brand_string") or platform.processor()


def gcp(path):
    try:
        req = urllib.request.Request(f"http://metadata.google.internal/computeMetadata/v1/instance/{path}", headers={"Metadata-Flavor": "Google"})
        return urllib.request.urlopen(req, timeout=1).read().decode().rsplit("/", 1)[-1]
    except Exception:
        return ""


def stat():
    """user+nice+system+idle+iowait+irq+softirq+steal, and steal, in jiffies."""
    if not os.path.exists("/proc/stat"):
        return None
    f = [int(x) for x in open("/proc/stat").readline().split()[1:9]]
    return sum(f), f[7]


def npm_versions():
    out = {}
    for name in ["tiktoken", "gpt-tokenizer", "js-tiktoken"]:
        p = os.path.join(ROOT, "js", "bench", "node_modules", name, "package.json")
        if os.path.exists(p):
            out[f"{name} (npm)"] = json.load(open(p))["version"]
    return out


def start(run, work, core):
    py = os.environ.get("PY", sys.executable)
    versions = {
        "rustc": sh("rustc", "--version").split(" ")[1:2],
        "python": sh(py, "-c", "import sys; print(sys.version.split()[0])"),
        "tiktoken (PyPI)": sh(py, "-c", "import tiktoken; print(tiktoken.__version__)"),
        "node": sh("node", "--version").lstrip("v"),
        "go": sh("go", "env", "GOVERSION").removeprefix("go"),
    }
    versions["rustc"] = versions["rustc"][0] if versions["rustc"] else ""
    versions.update(npm_versions())
    gov = "/sys/devices/system/cpu/cpu%s/cpufreq/scaling_governor" % core
    m = dict(
        started=datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds"),
        cpu=cpu_model(),
        arch=platform.machine(),
        cores=os.cpu_count(),
        instance=gcp("machine-type") or "not a GCP instance",
        kernel=platform.release(),
        governor=open(gov).read().strip() if os.path.exists(gov) else "not exposed",
        smt=open("/sys/devices/system/cpu/smt/active").read().strip() if os.path.exists("/sys/devices/system/cpu/smt/active") else "?",
        core=core,
        rustflags=os.environ.get("RUSTFLAGS", ""),
        commit=sh("git", "rev-parse", "HEAD", cwd=ROOT),
        dirty=bool(sh("git", "status", "--porcelain", "--untracked-files=no", "--", ".", ":!bench/results", cwd=ROOT)),
        versions={k: v for k, v in versions.items() if v},
        workloads={os.path.basename(p): hashlib.sha256(open(p, "rb").read()).hexdigest()
                   for p in sorted(glob.glob(os.path.join(work, "*.json")))},
        _stat=stat(),
    )
    os.makedirs(run, exist_ok=True)
    json.dump(m, open(os.path.join(run, "machine.json"), "w"), indent=1)


def end(run):
    p = os.path.join(run, "machine.json")
    m = json.load(open(p))
    s0, s1 = m.pop("_stat", None), stat()
    if s0 and s1 and s1[0] > s0[0]:
        m["steal_pct"] = round(100 * (s1[1] - s0[1]) / (s1[0] - s0[0]), 2)
    m["ended"] = datetime.datetime.now(datetime.timezone.utc).isoformat(timespec="seconds")
    m["short"] = f"{m['instance']} ({m['cpu'].split('@')[0].strip()})"
    json.dump(m, open(p, "w"), indent=1)


if __name__ == "__main__":
    if sys.argv[1] == "start":
        start(sys.argv[2], sys.argv[3], sys.argv[4])
    else:
        end(sys.argv[2])
