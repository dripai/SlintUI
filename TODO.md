# SlintUI 待办清单

本文是仓库未完成工作的唯一状态清单。目标组件范围和优先级见 [`design/component-inventory.md`](design/component-inventory.md)，已经实现的组件 API 与明确限制见 [`design/component-specs/`](design/component-specs/README.md)。不再按 P0、P1、P2 分别维护状态文档。

最近核对：2026-07-20。当前基线已实现全部 P0、提前完成的 `ScrollArea`、里程碑 D 的 11 份 P1 规格和全部 20 个 P2 组件，共有 58 份已实现组件规格；按组件清单行计，当前共有 12 个 P1 条目已经实现。`v0.1.0` 标签已经发布。

## 1. 现有组件质量补齐

这些任务优先于继续增加组件数量。完成一项后必须同步组件规格、Gallery、测试和截图证据。

- [ ] 按 [`design/component-inventory.md`](design/component-inventory.md) 第 13 节逐个复核 58 个已实现组件，不以“已经导出”替代完成定义。
- [ ] 为每个组件补齐适用的 Variant、Size、Disabled、Loading、Selected、Error、Focus 和长文本示例；不适用状态在规格中明确说明。
- [ ] 把 Gallery 中通用的 API 与 Theme Token 占位说明替换为组件特有内容。
- [ ] 扩充交互测试，使每个有状态组件至少覆盖正常、禁用、重复操作和边界路径。
- [ ] 建立文本、图标、边框、焦点环和状态色的对比度检查记录。
- [ ] 更新 125% 和 200% 代表性截图；继续保留主题、密度、Locale、RTL 和 reduced-motion 组合检查。

## 2. 剩余 P1 组件（28 项）

只有出现至少两个明确产品场景，或进入经过确认的新里程碑后才开始实现。

### 操作（3）

- [ ] `ButtonGroup`
- [ ] `ToggleButton`
- [ ] `Link`

### 布局与容器（4）

- [ ] `Flex`
- [ ] `Grid`
- [ ] `Panel`
- [ ] `AppShell`

### 导航（1）

- [ ] `Dropdown`

### 数据录入（8）

- [ ] `TextArea`
- [ ] `SearchField`
- [ ] `NumberInput`
- [ ] `CheckboxGroup`
- [ ] `Radio` / `RadioGroup`
- [ ] `ComboBox`
- [ ] `Slider`
- [ ] `FilePicker`

### 数据展示（6）

- [ ] `Badge`
- [ ] `Tag`
- [ ] `List` / `ListItem`
- [ ] `DescriptionList`
- [ ] `Collapse`
- [ ] `Popover`

### 反馈（4）

- [ ] `Alert`
- [ ] `ProgressRing`
- [ ] `ResultState`
- [ ] `LoadingOverlay`

### 桌面核心（2）

- [ ] `ShortcutHint`
- [ ] `TitleBar`

## 3. 平台增强（6 项）

平台增强必须进入独立 crate，并为 Windows、macOS、Linux 保持统一公开语义；核心 `.slint` 组件不得直接调用系统 API。

- [ ] 窗口拖动与缩放
- [ ] 原生窗口阴影
- [ ] 系统托盘
- [ ] 全局快捷键
- [ ] 原生文件对话框
- [ ] 系统通知

## 4. 平台与无障碍验证

- [ ] Windows：原生 GPU 后端、系统 DPI、Narrator、NVDA、多种输入法和原生弹层真机验证。
- [ ] macOS：编译、字体、焦点、VoiceOver、DPI 和原生弹层真机验证。
- [ ] Linux：编译、字体、焦点、Orca、DPI 和原生弹层真机验证。
- [ ] 记录各平台失败项、豁免原因、复现环境和修复版本，不以语义声明代替真机结果。

## 5. 产品接入与稳定发布

- [ ] 由首个真实业务项目通过固定 Git tag 接入 SlintUI。
- [ ] 迁移一个真实设置页，验证表单、错误、Locale、RTL 和缩放。
- [ ] 迁移一个真实工具栏，验证高频操作、快捷键、Tooltip 和禁用状态。
- [ ] 将接入问题回收到 SlintUI 修复，禁止业务项目复制组件源码后二次修改。
- [ ] 根据首个产品反馈收敛公开 API，并建立 Changelog 与升级说明。
- [ ] 接入第二个业务项目，验证跨产品复用。
- [ ] 完成兼容性审查后发布 `1.0.0`。

## 6. 已知限制与上游依赖

以下项目不是静默缺失的功能。只有 Slint 提供稳定能力，或经过评审决定最小必要自定义方案后，才转为实施任务。

- Tooltip 的延迟、偏移和通用焦点触发。
- Toolbar 任意子项的 roving focus、方向键遍历和自动溢出。
- 任意 `@children` 的运行时方向切换和自动换行。
- 任意业务组合的自动 RTL 结构镜像和方向图标镜像。
- 通用跨窗口焦点陷阱、所有者焦点恢复和平台模态关系。
- `ScrollArea` 虚拟化；超长数据集合继续使用 Table、DataGrid 或后续 List 协议。

## 7. 不进入当前待办

FloatButton、Affix、Anchor、Carousel、Tour、Watermark、QRCode、Rate、Mentions、Transfer、Masonry、网页 24 栅格和 ProComponents 没有明确桌面产品需求，不作为当前完成度缺口。出现真实场景时先更新组件清单，再进入本 TODO。
