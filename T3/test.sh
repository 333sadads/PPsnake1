#!/bin/bash

# 切换到Windows路径（在Git Bash/WSL中使用/mnt/转换）
cd /mnt/c/Users/27352/Desktop/rustPP/T3

total=0

for ((i=1; i<=100; i++))
do
    # 运行npm命令并捕获最后一行
    output=$(npm run --silent submit-test 2>&1)
    # 提取最后一行并过滤掉npm通知
    last_line=$(echo "$output" | grep -v '^npm notice' | tail -n 1)
    # 提取所有连续数字（支持多位数结果）
    result=$(echo "$last_line" | grep -oE '[0-9]+' | head -n 1)
    
    # 安全地进行算术运算
    if [[ "$result" =~ ^[0-9]+$ ]]; then
        total=$((total + result))
        echo "Run $i: Result = $result, Total = $total"
    else
        echo "Run $i: Invalid result '$result', skipping..."
    fi
done

echo "Final sum of results (0/1): $total"

read -p "Press Enter to exit..."