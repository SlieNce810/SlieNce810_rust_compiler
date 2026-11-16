#!/usr/bin/env bash
# 批量跑 tests/cases/*.sy，与同名 .out 逐字节比对。
#
#   ./tests/run_cases.sh                # 跑全部（有 .out 的才比对，没有的只打印输出）
#   ./tests/run_cases.sh sample3        # 只跑指定用例（不带 .sy 后缀也行）
#   ./tests/run_cases.sh --golden       # 给所有缺 .out 的用例生成金标准（谨慎！先自己看眼输出）
#
# 退出码：全过 0，有失败 1。

set -uo pipefail

cd "$(dirname "$0")/.."

BIN=./target/release/compiler
if [[ ! -x $BIN ]]; then
    echo "找不到 $BIN，先跑 cargo build --release" >&2
    exit 1
fi

make_golden=0
args=()
for a in "$@"; do
    case "$a" in
        --golden) make_golden=1 ;;
        *) args+=("$a") ;;
    esac
done

if [[ ${#args[@]} -gt 0 ]]; then
    sources=()
    for name in "${args[@]}"; do
        sources+=("tests/cases/${name%.sy}.sy")
    done
else
    sources=(tests/cases/*.sy)
fi

pass=0
fail=0
skip=0
tmp=$(mktemp)

for src in "${sources[@]}"; do
    name=$(basename "$src" .sy)
    golden="tests/cases/$name.out"

    actual=$("$BIN" "$src" 2>&1)
    if [[ -n $actual ]]; then
        printf '%s\n' "$actual" > "$tmp"
    else
        : > "$tmp"
    fi

    if [[ ! -f $golden ]]; then
        if [[ $make_golden -eq 1 ]]; then
            cp "$tmp" "$golden"
            printf '[GOLD] %s  (%s 行)\n' "$name" "$(wc -l < "$golden")"
        else
            printf '[DUMP] %s  —— 还没有金标准，输出如下：\n' "$name"
            sed 's/^/       /' "$tmp"
        fi
        skip=$((skip + 1))
        continue
    fi

    if diff -u "$golden" "$tmp" > "${tmp}.diff" 2>&1; then
        printf '[PASS] %s\n' "$name"
        pass=$((pass + 1))
    else
        printf '[FAIL] %s\n' "$name"
        sed 's/^/       /' "${tmp}.diff"
        fail=$((fail + 1))
    fi
done

rm -f "$tmp" "${tmp}.diff"

printf '\n通过 %d，失败 %d，无金标准 %d\n' "$pass" "$fail" "$skip"
[[ $fail -eq 0 ]]
