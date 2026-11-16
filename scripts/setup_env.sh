#!/usr/bin/env bash
# Lab0 环境补齐 / 自检脚本（在 WSL Ubuntu-22.04 的 bash 里执行）
#
# 用法：
#   bash scripts/setup_env.sh           # 安装 + 写 ~/.bashrc + 自检
#   bash scripts/setup_env.sh --check   # 只自检，不改系统（装完后再跑用这个）
#
# 注意：不要在 PowerShell 里跑。PowerShell 不认 `&&` 和 `source`，
#       先 `wsl` 进发行版，或 `bash -lc "..."` 包一层。
set -euo pipefail

PKGS="llvm-14 llvm-runtime clang-14 zlib1g-dev libtinfo-dev"
PREFIX="/usr/lib/llvm-14"
MARK="# >>> lab0-llvm (demo_compiler) >>>"

# llvm-14       -> /usr/lib/llvm-14/bin/llvm-config（llvm-sys 构建时必需）
# llvm-runtime  -> lli（验证生成的 IR 能不能跑）
# clang-14      -> 讲义要求；也方便手写 IR / 反汇编
# zlib1g-dev    -> llvm-config --system-libs 里的 -lz
# libtinfo-dev  -> llvm-config --system-libs 里的 -ltinfo（少了链接期报 cannot find -ltinfo）

CHECK_ONLY=0
[ "${1:-}" = "--check" ] && CHECK_ONLY=1

if [ "$CHECK_ONLY" -eq 0 ]; then
    echo "==> 1/3 安装: $PKGS"
    sudo apt-get update
    sudo apt-get install -y $PKGS

    echo "==> 2/3 写入 LLVM_SYS_140_PREFIX 到 ~/.bashrc"
    if grep -qF "$MARK" "$HOME/.bashrc"; then
        echo "    已存在，跳过"
    else
        {
            echo ""
            echo "$MARK"
            echo "export LLVM_SYS_140_PREFIX=$PREFIX"
            echo "# <<< lab0-llvm <<<"
        } >> "$HOME/.bashrc"
        echo "    已追加到 ~/.bashrc（新终端生效；当前终端执行: source ~/.bashrc）"
    fi
else
    echo "==> 1-2/3 跳过安装（--check 模式）"
fi
export LLVM_SYS_140_PREFIX="$PREFIX"

echo "==> 3/3 自检"
# 注意：命令和参数必须分开传（check "llvm-config" "$PREFIX/bin/llvm-config" --version），
# 写成一整条字符串会被当成程序名，必然报 "No such file or directory"。
check() { # check <名字> <命令> [参数...]
    local name="$1"; shift
    local out first rc
    out=$("$@" 2>&1) && rc=0 || rc=$?
    first=$(printf '%s\n' "$out" | head -1)
    if [ "$rc" -eq 0 ]; then
        printf '  [ok]   %-16s %s\n' "$name" "$first"
    else
        printf '  [MISS] %-16s %s\n' "$name" "$first"
    fi
}

check "llvm-config" "$PREFIX/bin/llvm-config" --version
check "clang"       "clang-14" --version
check "lli"         "$PREFIX/bin/lli" --version
check "rustc"       "rustc" --version
check "cargo"       "cargo" --version
check "gcc"         "gcc" --version
check "make"        "make" --version

if [ -x "$PREFIX/bin/llvm-config" ]; then
    printf '  [ok]   %-16s %s\n' "LLVM_SYS 前缀" "$PREFIX/bin/llvm-config 存在"
else
    printf '  [MISS] %-16s %s\n' "LLVM_SYS 前缀" "$PREFIX/bin/llvm-config 不存在"
fi

echo
echo "自检若有 [MISS]，把上面输出发我。"
echo "通过后：cargo build  （首次会拉 pest / llvm-sys，需要网络）"
