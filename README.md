# Life-Death-Scheduler

## 调度简介

> Life-Death-Scheduler 是一个 Android 设备 CPU 动态调度器，以 Magisk 模块形式运行。它能识别不同应用并自动切换 CPU 频率策略，在性能和功耗间实现最佳平衡。

- 使用 Rust 语言开发，高性能实现
- 支持应用级别的动态调度
- 自动禁用系统默认的性能调节服务
- 提供四种电源策略模式

## 安装方法

1. 设备需已安装 **Magisk v20.3+**（推荐最新版）
2. 下载模块
3. Magisk → 模块 → 本地安装
4. 选择模块 zip 文件
5. 安装完成后必须重启设备
6. 开机后自动运行

## 使用说明

模块安装后自动工作，自定义配置步骤：

1. 修改配置文件：`/data/adb/modules/life_death_scheduler/config.toml`
2. 修改后实时生效，无需重启
3. 查看运行日志：`/data/adb/modules/life_death_scheduler/run.log`

## 配置文件详解

### 1. 全局模式设置 (osm)

```toml
osm = "powersave"  # 默认全局省电模式
```

**可选值**：

- `powersave`：省电模式
- `balance`：平衡模式
- `performance`：性能模式
- `fast`：极致性能模式

### 2. Debug 模式设置

```toml
debug = false  # 默认关闭
```

### 3. Binder 设置

```toml
binder = false  # 默认使用命令行获取前台应用
```

### 4. CPU 策略分配

```toml
[cpu_config]
big = 7          # 大核起始编号
middle = 4       # 中核起始编号
small = 0        # 小核起始编号（可选）
super_big = 9    # 超大核起始编号（可选）
```

### 5. 模式配置模板

```toml
[模式名称]  # 如 [performance]
#----------- 控制器设置 -----------
super_big_cpu_governor = "调速器名称"  # 超大核调速器（可选）
big_cpu_governor = "调速器名称"        # 大核调速器（必选）
middle_cpu_governor = "调速器名称"     # 中核调速器（必选）
small_cpu_governor = "调速器名称"      # 小核调速器（可选）

#----------- 频率设置 -----------
super_big_cpu_freq = 频率值           # 超大核频率（可选）
big_cpu_freq = 频率值                 # 大核频率（必选）
middle_cpu_freq = 频率值              # 中核频率（必选）
small_cpu_freq = 频率值               # 小核频率（可选）

#----------- CpuCtl 配置 -----------
[模式名称.cpuctl]
top_app = { shares = 数值, uclamp = { max = 数值, min = 数值 } }
foreground = { shares = 数值, uclamp = { max = 数值, min = 数值 } }
```

### 配置说明：

**CpuCtl 参数效果**：

- `shares`：CPU 时间片分配权重
- `uclamp.max`：最大 CPU 利用率限制
- `uclamp.min`：最小 CPU 利用率保障

#### 完整配置示例（performance 模式）

```toml
[performance]
super_big_cpu_governor = "performance"
big_cpu_governor = "performance"
middle_cpu_governor = "schedutil"
small_cpu_governor = "powersave"

super_big_cpu_freq = 2500000000
big_cpu_freq = 2300000000
middle_cpu_freq = 1900000000
small_cpu_freq = 1400000000

[performance.cpuctl]
top_app = { shares = 100, uclamp = { max = 100, min = 0 } }
foreground = { shares = 100, uclamp = { max = 100, min = 0 } }
```

### 配置规则：

1. 必须保持配置结构完整
2. 调速器名称使用英文双引号包裹
3. 频率值为整数（单位：Hz）
4. 设备无对应 CPU 核心时删除相关配置项

## 频率单位

所有频率参数单位为 **赫兹(Hz)**：

- 1 kHz = 1,000 Hz
- 1 MHz = 1,000,000 Hz
- 1 GHz = 1,000,000,000 Hz

## 策略优先级

1. applist 应用专属配置
2. osm 全局默认配置

## 工作原理

1. 禁用系统原生性能服务
2. 监控前台应用切换
3. 动态调整 CPU 策略
4. 通过内核接口控制 CPU 频率

## 常见问题

**应用未按预期切换模式？**

- 检查包名：`pm list packages | grep 关键词`
- 检查配置文件语法

**查看运行状态**

- `tail -f /data/adb/modules/life_death_scheduler/run.log`

**添加自定义应用**

```toml
[applist]
"com.tencent.tmgp.sgame" = "performance"  # 王者荣耀
"com.zhiliaoapp.musically" = "fast"       # 抖音
```

## 技术支持

Telegram 群组：https://t.me/+4qh_4BOWDTw3OTU1
l