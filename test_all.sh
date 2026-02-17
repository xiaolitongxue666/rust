#!/bin/bash
# 测试所有题目，RUSTFLAGS="-D warnings" 且 --include-ignored
# xorcism: --features io; doubly-linked-list: --features advanced

cd "$(dirname "$0")"
failed=""
warned=""

for d in */; do
  name="${d%/}"
  [[ "$name" == "bin" || "$name" == "concepts" || "$name" == "docs" || "$name" == "exercises" ]] && continue
  [[ ! -f "$name/Cargo.toml" ]] && continue

  if [[ "$name" == "xorcism" ]]; then
    out=$(cd "$name" && RUSTFLAGS="-D warnings" cargo test --features io -- --include-ignored 2>&1)
  elif [[ "$name" == "doubly-linked-list" ]]; then
    out=$(cd "$name" && RUSTFLAGS="-D warnings" cargo test --features advanced -- --include-ignored 2>&1)
  else
    out=$(cd "$name" && RUSTFLAGS="-D warnings" cargo test -- --include-ignored 2>&1)
  fi

  if ! echo "$out" | grep -q "test result: ok"; then
    failed="$failed $name"
    echo "FAIL: $name"
    echo "$out" | tail -40
    echo "================"
  elif echo "$out" | grep -q "warning:"; then
    warned="$warned $name"
    echo "WARN: $name"
    echo "$out" | grep "warning:"
    echo "================"
  fi
done

echo ""
echo "Failed:$failed"
echo "Warned:$warned"
