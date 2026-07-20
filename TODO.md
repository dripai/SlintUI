# SlintUI 待办清单

本文是仓库未完成工作的唯一状态清单。目标组件范围和优先级见 [`design/component-inventory.md`](design/component-inventory.md)，已经实现的组件 API 与明确限制见 [`design/component-specs/`](design/component-specs/README.md)。不再按 P0、P1、P2 分别维护状态文档。

最近核对：2026-07-20。当前基线已实现全部 P0、40 份 P1 和 20 份 P2 组件规格，共 86 份；原 58 个组件的完成定义复核及剩余 28 个 P1 条目均已完成。`v0.1.0` 是首个固定 P0 标签，当前未提交工作属于其后的完整核心组件基线。

## 1. 现有组件质量补齐

这些任务优先于继续增加组件数量。完成一项后必须同步组件规格、Gallery、测试和截图证据。

- [x] 按 [`design/component-inventory.md`](design/component-inventory.md) 第 13 节逐个复核 58 个已实现组件；证据见 [`design/component-quality-audit.md`](design/component-quality-audit.md)。
- [x] 为每个组件补齐适用的 Variant、Size、Disabled、Loading、Selected、Error、Focus 和长文本示例；不适用状态已在规格或质量记录中说明。
- [x] Gallery 已使用组件特有 API、状态和 Theme Token 说明，不再复用通用占位内容。
- [x] 交互 Harness 已覆盖全部有状态核心协议的正常、禁用、重复操作和边界路径。
- [x] 已建立文本、图标、边框、焦点环和状态色的 [`design/contrast-audit.md`](design/contrast-audit.md) 记录。
- [x] 已新增 125% 和 200% 代表性截图，并继续保留主题、密度、Locale、RTL 和 reduced-motion 组合检查。

## 2. P1 完整批次（28 项，已完成）

以下组件已作为里程碑 F 完成，并纳入公开入口、分类 Gallery、规格和质量门禁。

### 操作（3）

- [x] `ButtonGroup`
- [x] `ToggleButton`
- [x] `Link`

### 布局与容器（4）

- [x] `Flex`
- [x] `Grid`
- [x] `Panel`
- [x] `AppShell`

### 导航（1）

- [x] `Dropdown`

### 数据录入（8）

- [x] `TextArea`
- [x] `SearchField`
- [x] `NumberInput`
- [x] `CheckboxGroup`
- [x] `Radio` / `RadioGroup`
- [x] `ComboBox`
- [x] `Slider`
- [x] `FilePicker`

### 数据展示（6）

- [x] `Badge`
- [x] `Tag`
- [x] `List` / `ListItem`
- [x] `DescriptionList`
- [x] `Collapse`
- [x] `Popover`

### 反馈（4）

- [x] `Alert`
- [x] `ProgressRing`
- [x] `ResultState`
- [x] `LoadingOverlay`

### 桌面核心（2）

- [x] `ShortcutHint`
- [x] `TitleBar`

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
