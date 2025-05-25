# 💀 life-death-scheduler ⚡

## 📘 调度简介

> 🔧 life-death-scheduler 是一个 ✨ 超强魔法般的 Android 设备 CPU 动态调度器，以 Magisk 模块形式 🛸 运行！它能像 🔮 智能巫师一样识别不同应用，自动切换 CPU 频率策略，在 🚀 性能怪兽和 🐢 省电乌龟间找到完美平衡！

- 🦾 使用 Rust 语言开发，性能堪比光速
- 🔄 支持应用级别的动态调度，像 🕵️‍♂️ 特工般精准识别
- 🚫 自动禁用系统默认的性能调节服务，像 🗡️ 斩断枷锁
- 🎛️ 提供四种电源策略模式，满足从 📖 文青到 🎮 电竞玩家的所有需求

## 📥 安装方法

1.  🔐 设备需已安装 **Magisk v20.3+**（建议用最新版）
2.  🎁 下载模块：点击右上角 🌟 星星收藏时自动获取最新版
3.  🧙 打开 Magisk → 模块 → 📤 本地安装
4.  📎 选择下载的 zip 包 → 🪄 魔法启动！
5.  🔄 安装完成必须重启 → 见证奇迹的时刻！
6.  ✅ 开机后自动运行

## 🛠️ 使用说明

安装后模块会像 🤖 勤劳机器人自动工作！想自定义？请按以下步骤：

1.  📝 修改配置文件：`/data/adb/modules/life_death_scheduler/config.toml`

- 推荐使用 🏹MT 管理器或 🍜 酷安下载的编辑器

2.  📡 修改后实时生效，无需重启 → 就像 🌪️ 龙卷风一样快！
3.  📜 查看日志：`/data/adb/modules/life_death_scheduler/run.log`

- 遇到问题先看这里，像 🔍 侦探一样排查！

## 📂 配置文件详解

### 1. 🌍 全局模式设置 (osm)

```toml
osm = "powersave"  # 默认全局省电模式
```

- 🎛️ **作用**：定义默认电源策略（applist 指定的应用除外）
- 🌈 **可选值**：
  - `powersave` → 🐢 龟速省电模式
  - `balance` → ⚖️ 智能平衡模式
  - `performance` → 🚀 性能爆发模式
  - `fast` → 🏎️ 赛道极速模式

### 2. 🎚️ CPU 策略分配 (cpu_config)

```toml
[cpu_config]
big = 7          # 🐯大核：火力全开！
middle = 4       # 🐱中核：灵活调度
small = 0        # 🐭小核：养老模式（可选）
super_big = 9    # 🦁超大核：核弹启动！（可选）
```

- ⚙️ **作用**：定义不同策略对应的 CPU 核心
- 🔧 **字段说明**：
  - `big`: 大核 CPU 编号起始（范围 0-7）
  - `middle`: 中核 CPU 编号起始（范围 0-4）
  - `small`: 小核 CPU 编号起始（可选）
  - `super_big`: 超大核 CPU 编号起始（可选）

### 3. 🎛️ 模式详情

每个模式(`powersave`/`balance`/`performance`/`fast`)都像 🤖 机器人有固定骨架，必须按以下结构配置：

#### 模式配置模板

```toml
[模式名称]  # 比如 [performance]
#----------- 控制器设置 -----------
super_big_cpu_governor = "调速器名称"  # 🦁超大核方向盘（可选）
big_cpu_governor = "调速器名称"        # 🐯大核油门踏板（必选）
middle_cpu_governor = "调速器名称"     # 🐱中核变速器（必选）
small_cpu_governor = "调速器名称"      # 🐭小核节能开关（可选）

#----------- 频率设置 -----------
super_big_cpu_freq = 频率值           # 🦁超大核转速表（可选）
big_cpu_freq = 频率值                 # 🐯大核涡轮压力（必选）
middle_cpu_freq = 频率值              # 🐱中核巡航速度（必选）
small_cpu_freq = 频率值               # 🐭小核怠速限制（可选）
```

#### 🚦 参数规则

| 参数类型                | 必填规则                          | 示例值          |
| ----------------------- | --------------------------------- | --------------- |
| `*_cpu_governor`        | 有对应 CPU 核心时必须填写         | `"schedutil"`   |
| `*_cpu_freq`            | 有对应 CPU 核心时必须填写         | `2300000000`    |
| `super_big`/`small`参数 | 设备没有该类型核心时请删除整行 ❗ | 无核心就别写啦~ |

#### CpuCtl 配置模板

```toml
[powersave.cpuctl]
# 🎮大Boss应用资源控制（微信/游戏等）
top_app = { shares = 0, uclamp = { max = 0, min = 0 } }  # 🔒禁止占用CPU大核，锁死性能闸门

# 🖥️前台小秘书资源控制（当前操作的应用）
foreground = { shares = 0, uclamp = { max = 0, min = 0 } }  # ✂️剪掉性能翅膀，强制节能模式
```

### 配置效果说明：

- `shares = 0`：🗿 将应用 CPU 时间片权重设为最低值，相当于给应用戴上"缓速器"
- `uclamp.max = 0`：🔋 限制任务最大 CPU 利用率到最低档（约 0-20%），像给油门踏板加装限位器
- `uclamp.min = 0`：🌱 取消最低性能保障，允许系统彻底放飞自我降频，宛如给 CPU 穿上草鞋轻装出行

### 组合技效果：

当进入`powersave`模式时，系统会：  
🐌 **双重枷锁**：前台和顶层应用被套上性能限制器，强制进入"低功耗囚笼"  
🌌 **黑洞级节能**：所有任务只能在最低频率区间运行，CPU 像进入省电冬眠状态  
🔋 **续航暴增**：适合长时间阅读/待机，电量消耗速度堪比树懒移动（但应用可能会卡成 PPT 哦~）

#### 🌰 完整示例（performance 模式）

```toml
[performance]
# 🎛️控制器配置
super_big_cpu_governor = "performance"  # 🦁超大核狂暴模式
big_cpu_governor = "performance"        # 🐯大核火力全开
middle_cpu_governor = "schedutil"       # 🐱中核智能调度
small_cpu_governor = "powersave"        # 🐭小核躺平节能

# ⚡频率配置
super_big_cpu_freq = 2500000000         # 🦁2.5GHz核弹预备
big_cpu_freq = 2300000000               # 🐯2.3GHz涡轮增压
middle_cpu_freq = 1900000000            # 🐱1.9GHz稳定输出
small_cpu_freq = 1400000000             # 🐭1.4GHz省电养老

# CpuCtl配置
[performance.cpuctl]
top_app = { shares = 100, uclamp = { max = 100, min = 0 } }
foreground = { shares = 100, uclamp = { max = 100, min = 0 } }
```

#### ❗ 重要提示

1. 结构像 🧱 乐高积木不能拆改，否则模块会 🤯 崩溃
2. 用英文双引号`""`包裹调速器名称，像给字符串穿 👗 裙子
3. 频率值直接写数字，像 🔢 裸奔的数字战士
4. 没有的 CPU 类型请整个参数删除，像 🗑️ 扔掉不需要的零件

## 📏 频率单位说明

所有频率参数单位为 **赫兹(Hz)**，换算指南：

- 1 kHz = 1,000 Hz → 🐜 蚂蚁速度
- 1 MHz = 1,000,000 Hz → 🐇 兔子跳跃
- 1 GHz = 1,000,000,000 Hz → 🚀 火箭升空

## 🏆 策略优先级

1. 🥇 专属 VIP 通道：applist 应用配置
2. 🌐 普通通道：osm 全局默认配置

## ⚙️ 工作原理

1.  🚫 禁用系统服务：像 🗡️ 刺客干掉 miuibooster、perfd 等进程
2.  👁️ 前台监控：像 🦉 猫头鹰一样紧盯应用切换
3.  🔄 动态调整：像 🎭 川剧变脸一样切换 CPU 策略
4.  📡 底层控制：像 🕹️ 游戏手柄精准操作 CPU 频率

## 🆘 常见问题

- ❓ **应用没按预期切换模式？**

  - ✅ 检查包名：`pm list packages | grep 关键词`
  - ✅ 检查配置文件是否有 🍗 鸡腿（语法错误）

- ❓ **如何查看运行状态？**
  - 📜 实时日志：`tail -f /data/adb/modules/life_death_scheduler/run.log`
- ❓ **想添加更多应用？**
  - 📦 编辑 config.toml 的 [applist] 部分：
  ```toml
  [applist]
  "com.tencent.tmgp.sgame" = "performance"  # 🎮王者荣耀：满血模式
  "com.zhiliaoapp.musically" = "fast"      # 🎵抖音：丝滑体验
  ```

## 📞 技术支持

遇到问题？快来加入我们的 💬 魔法交流群：

- 🔢 Telegram: https://t.me/+4qh_4BOWDTw3OTU1

（⚠️ 警告：使用此模式可能导致设备变身 🍟 炸薯条机器）
