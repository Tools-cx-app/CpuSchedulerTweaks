
<div align="center">

# **life-death-scheduler**

### Android CPU 智能调度器 - 生死之间的性能平衡

[![Rust][rust-badge]][rust-url]
[![Version][version-badge]][version-url]
[![Release][release-badge]][release-url]
[![Download][download-badge]][download-url]
[![Telegram][telegram-badge]][telegram-url]

</div>

[rust-badge]: https://img.shields.io/badge/Language-Rust-orange.svg?style=for-the-badge&logo=rust
[rust-url]: https://www.rust-lang.org/
[version-badge]: https://img.shields.io/badge/Version-1.5.0-blue.svg?style=for-the-badge
[version-url]: https://github.com/Tools-cx-app/life-death-scheduler/releases/latest
[release-badge]: https://img.shields.io/github/v/release/Tools-cx-app/life-death-scheduler?style=for-the-badge&logo=rust
[release-url]: https://github.com/Tools-cx-app/life-death-scheduler-release/releases/latest
[download-badge]: https://img.shields.io/github/downloads/Tools-cx-app/life-death-scheduler/total?style=for-the-badge
[download-url]: https://github.com/Tools-cx-app/life-death-scheduler-release/releases/latest
[telegram-badge]: https://img.shields.io/badge/Group-blue?style=for-the-badge&logo=telegram&label=Telegram
[telegram-url]: https://t.me/ld_scheduler

## **简介**

> 在性能与功耗的生死边缘，寻找完美平衡。`Life-Death-Scheduler` 是一个高性能的 Android CPU 动态调度器，通过智能识别前台应用并实时调整 CPU 策略，在保证流畅体验的同时最大化电池续航

- ### **什么是`life-death-scheduler`?**

  - `life-death-scheduler`是运行在用户态的智能 CPU 调度器实现，基于 Rust 语言开发，具有极佳的性能和兼容性优势

## **自定义(配置)**

- ### **配置路径: `/data/adb/modules/life_death_scheduler/config.toml`**

- ### **参数(`全局设置`)说明:**

  - **osm**

    - 类型: `字符串`
    - `"powersave"`: 省电模式 \*
    - `"balance"`: 平衡模式
    - `"performance"`: 性能模式
    - `"fast"`: 极速模式

  - **binder**

    - 类型: `bool`
    - `true`: 使用 Binder 通信获取前台应用
    - `false`: 使用命令行方式获取前台应用 \*

  - **debug**

    - 类型: `bool`
    - `true`: 启用调试模式
    - `false`: 关闭调试模式 \*

  - `*`: 默认配置

- ### **CPU 核心配置(`cpu_config`)说明:**

  - **`big` = `核心编号`**

    - `big`: 大核起始编号
    - `middle`: 中核起始编号
    - `small`: 小核起始编号(可选)

- ### **应用列表(`applist`)说明:**

  - **`"package"` = `"mode"`**

    - `package`: 字符串，应用包名
    - `mode`: 字符串，该应用使用的性能模式，`life-death-scheduler`会在检测到该应用时自动切换到对应模式

- ### **模式(`powersave` / `balance` / `performance` / `fast`)说明:**

  - #### **模式切换:**

    - 通过修改配置文件中的 `osm` 参数切换全局默认模式
    - 应用专属配置在 `applist` 中设置，优先级高于全局模式

  - #### **模式参数说明:**

    - **freqs:**
      - 支持格式: `cpu_type = { max = <频率>, min = <频率> }`
      - 解释: 设置对应 CPU 核心的最大和最小频率限制(单位: Hz)

    - **governor:**

      - 类型: `字符串`
      - 常用值: `"schedutil"`, `"performance"`, `"powersave"`, `"ondemand"`
      - 解释: 设置 CPU 调速器策略

    - **cpuctl:**

      - 格式: `top_app = { shares = <数值>, uclamp = { max = <数值>, min = <数值> } }`
      - 解释: 设置 CPU 控制组参数，控制应用的 CPU 资源分配

### **`config.toml`配置标准例:**

```toml
osm = "balance"
binder = false
debug = false

[cpu_config]
big = 7
middle = 4
small = 0

[powersave.freqs]
big_cpu = { max = 1800000, min = 500000 }
middle_cpu = { max = 1600000, min = 500000 }
small_cpu = { max = 1400000, min = 500000 }

[powersave.governor]
big_cpu = "schedutil"
middle_cpu = "schedutil"
small_cpu = "schedutil"

[powersave.cpuctl]
top_app = { shares = 0, uclamp = { max = 0, min = 0 } }

[balance.freqs]
big_cpu = { max = 2200000, min = 800000 }
middle_cpu = { max = 2000000, min = 800000 }
small_cpu = { max = 1800000, min = 800000 }

[balance.governor]
big_cpu = "schedutil"
middle_cpu = "schedutil"
small_cpu = "schedutil"

[balance.cpuctl]
top_app = { shares = 0, uclamp = { max = 0, min = 0 } }

[performance.freqs]
big_cpu = { max = 2800000, min = 1200000 }
middle_cpu = { max = 2500000, min = 1200000 }
small_cpu = { max = 2200000, min = 1200000 }

[performance.governor]
big_cpu = "schedutil"
middle_cpu = "schedutil"
small_cpu = "schedutil"

[performance.cpuctl]
top_app = { shares = 0, uclamp = { max = 0, min = 0 } }

[fast.freqs]
big_cpu = { max = 3200000, min = 1500000 }
middle_cpu = { max = 2800000, min = 1500000 }
small_cpu = { max = 2500000, min = 1500000 }

[fast.governor]
big_cpu = "schedutil"
middle_cpu = "schedutil"
small_cpu = "schedutil"

[fast.cpuctl]
top_app = { shares = 0, uclamp = { max = 0, min = 0 } }

[applist]
"bin.mt.plus" = "powersave"
"com.tencent.tmgp.sgame" = "performance"
"com.miHoYo.Yuanshen" = "fast"
"com.zhiliaoapp.musically" = "balance"
```
