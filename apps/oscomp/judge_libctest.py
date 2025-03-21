import json
import sys

# TODO: Add more commands to test here
libctest_baseline_out = {
    "libctest static fdopen": 1,
    "libctest static lseek_large": 1,
}

def parse_libctest(output):
    ans = {}
    key = ""
    for line in output.split("\n"):
        line = line.replace('\n', '').replace('\r', '')
        if "START /musl/entry-static.exe" in line or "START /glibc/entry-static.exe" in line:
            key = "libctest static " + line.split(" ")[3]
        elif "START /musl/entry-dynamic.exe" in line or "START /glibc/entry-dynamic.exe" in line:
            key = "libctest dynamic " + line.split(" ")[3]
        if line == "Pass!" and key != "":
            ans[key] = 1
    return ans

serial_out = sys.stdin.read()
libctest_output = parse_libctest(serial_out)
for k in libctest_baseline_out.keys():
    if k not in libctest_output:
        libctest_output[k] = 0

results = [{
    "name": k,
    "pass": v,
    "total": 1,
    "score": v,
} for k, v in libctest_output.items()]
for r in results:
    if r["score"] == 0:
        print(f"libctest testcase {r['name']} failed")
        exit(255)

print("libctest testcases passed")
print(json.dumps(results))