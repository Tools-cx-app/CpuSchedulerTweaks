# 💀 life-death-scheduler ⚡

## 📘 调度简介

> 🔧 life-death-scheduler 是一个强大的 Android 设备 CPU 动态调度器，以 Magisk 模块形式运行，能够根据不同应用自动切换 CPU 频率策略，优化设备性能与电池寿命的平衡。

- ✨ 使用 Rust 语言开发，高效且稳定
- 🔄 支持应用级别的动态调度
- 🛡️ 自动禁用系统默认的性能调节服务
- 🔋 提供多种电源策略模式，满足不同场景需求

## 📥 安装方法

1.  📱 确保您的设备已安装 **Magisk v20.3+**
2.  📦 下载最新版本的 life-death-scheduler 模块 zip 包
3.  🧙‍♂️ 在 Root 管理器中选择"模块"→"从本地安装"
4.  📂 选择下载好的 zip 包进行安装
5.  🔄 安装完成后重启设备
6.  ✅ 模块将在设备启动后自动运行

## 🛠️ 使用说明

安装后，life-death-scheduler 会自动根据配置文件调整 CPU 频率。您可以通过修改配置文件来自定义调度行为：

1.  📝 配置文件位置：`/data/adb/modules/life_death_scheduler/config.toml`
2.  🔍 修改配置后无需重启，模块会自动检测并应用新配置
3.  📊 日志文件位置：`/data/adb/modules/life_death_scheduler/run.log`

## 📂 配置文件详解

### 1. 🌐 全局模式设置 (osm)

```toml
osm = "powersave"
```

- **作用**：定义默认应用的电源策略（applist 指定的应用除外）
- **可选值**：`powersave`/`balance`/`performance`/`fast`

### 2. 🎛️ CPU 策略分配 (cpu_config)

```toml
[cpu_config]
big = 7
middle = 4
small = 0 # small可选，如果有
super_big = 9 # super_big可选，如果有
```

- **作用**：定义不同策略类型对应的 CPU 核心编号
- **字段说明**：
  - `big`: 大核 CPU 编号上限（核心 0-7）
  - `middle`: 中核 CPU 编号上限（核心 0-4）
  - `small`: 小核 CPU 编号上限（核心 0），如果设置了，请在配置文件设置对应频率
  - `super_big`: 超大核 CPU 编号上限（核心 0-7），如果设置了，请在配置文件设置对应频率

### 3. ⚡ 电源策略配置

#### 3.1 🌱 节能模式 (powersave)

```toml
[powersave]
big_cpu_freq = { max = 1800000, min = 500000 }   # 🐢 1.8GHz ~ 0.5GHz
middle_cpu_freq = { max = 1600000, min = 500000 } # 🐢 1.6GHz ~ 0.5GHz
small_cpu_freq = { max = 1400000, min = 500000 }  # 🐢 1.4GHz ~ 0.5GHz
```

#### 3.2 ⚖️ 均衡模式 (balance)

```toml
[balance]
big_cpu_freq = { max = 2200000, min = 800000 }   # 🚶 2.2GHz ~ 0.8GHz
middle_cpu_freq = { max = 2000000, min = 800000 } # 🚶 2.0GHz ~ 0.8GHz
small_cpu_freq = { max = 1800000, min = 800000 }  # 🚶 1.8GHz ~ 0.8GHz
```

#### 3.3 🚀 性能模式 (performance)

```toml
[performance]
big_cpu_freq = { max = 2800000, min = 1200000 }  # 💨 2.8GHz ~ 1.2GHz
middle_cpu_freq = { max = 2500000, min = 1200000 } # 💨 2.5GHz ~ 1.2GHz
small_cpu_freq = { max = 2200000, min = 1200000 } # 💨 2.2GHz ~ 1.2GHz
```

#### 3.4 🔥 极限模式 (fast)

```toml
[fast]
big_cpu_freq = { max = 3200000, min = 1500000 }  # 🚒 3.2GHz ~ 1.5GHz
middle_cpu_freq = { max = 2800000, min = 1500000 } # 🚒 2.8GHz ~ 1.5GHz
small_cpu_freq = { max = 2500000, min = 1500000 } # 🚒 2.5GHz ~ 1.5GHz
```

### 4. 📱 应用专属配置 (applist)

```toml
[applist]
"bin.mt.plus" = "powersave"
```

- **作用**：为指定应用设置专属电源策略
- **格式**：`"应用包名" = "策略模式"`
- **示例说明**：当启动 `bin.mt.plus`（MT 管理器）时，自动切换为 🌱 节能模式

## 📏 频率单位说明

所有频率参数单位为 **赫兹(Hz)**，配置时需注意：

- 1 kHz = 1,000 Hz ➡️ 🐜
- 1 MHz = 1,000,000 Hz ➡️ 🐇
- 1 GHz = 1,000,000,000 Hz ➡️ 🐆

## 🔝 策略优先级

1. 📌 applist 应用专属配置
2. 🌍 osm 全局默认配置

## 🔄 工作原理

1.  🚫 禁用系统默认的性能调节服务（如 miuibooster、perfd 等）
2.  👀 监控前台应用变化
3.  📊 根据配置文件和当前前台应用自动切换 CPU 频率策略
4.  ⚙️ 通过修改系统底层实现频率控制

## 🆘 常见问题

- ❓ **为什么某些应用没有按照预期切换模式？**

  - 请确认应用包名是否正确，可通过 `pm list packages | grep 关键词` 命令查询

- ❓ **如何查看当前运行状态？**

  - 查看日志文件：`cat /data/adb/modules/life_death_scheduler/run.log`

- ❓ **如何添加更多应用到专属配置？**
  - 在 `config.toml` 的 `[applist]` 部分添加 `"应用包名" = "策略模式"`

## 📞 反馈与支持

如有问题或建议，请加入反馈群：687235389
