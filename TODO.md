# SlintUI 待办清单

本文只记录尚未完成的工作。公开组件范围、分类和当前契约见 [`design/component-specs/`](design/component-specs/README.md)；历史 P0/P1/P2 里程碑不再作为成熟度依据。

最近核对：2026-07-20。当前公开入口包含 88 个组件和 1 个 `Theme` 全局对象，均按 `Alpha` 重新评审。“源码存在、可以编译、Gallery 可见”不表示达到生产级或 Beta。

## 1. 生产级 API 收敛

- [x] 将组件规格按 General、Layout、Navigation、Data Entry、Data Display、Feedback、Desktop 分类。
- [x] 为每个公开组件或全局对象建立独立规格，共 89 份。
- [x] 每份规格记录当前实际属性、数据类型、内容入口、事件、方法、Theme Token 和现有行为，不创建空 API 类别。
- [ ] 评审第一批 Button、IconButton、TextField、TextArea、Checkbox、Radio、Switch、Select 的状态所有权、事件时序和方法语义。
- [ ] 按评审结果修正第一批源码、Gallery、交互测试和截图，不保留未经使用验证的 Alpha 兼容接口。
- [ ] 依次评审 Form、Tabs、PopupMenu、Dropdown、ModalDialog、ConfirmDialog、Toast。
- [ ] 依次评审 List、ListItem、Table、Tree、DataGrid。
- [ ] 按组件风险继续评审其余 Alpha 规格，并逐项完成实现与验证闭环。
- [ ] 建立规格与 `index.slint` 的自动一致性检查，防止公开属性、事件、方法或导出发生未记录漂移。

## 2. Gallery 与测试

- [ ] 单组件页面只展示一个组件，并逐项覆盖该组件实际拥有的 API、视觉和行为规范。
- [ ] 为受控属性展示宿主赋值与用户操作的差异。
- [ ] 为实际公开的事件增加名称、参数、来源和累计次数日志。
- [ ] 为实际公开的方法增加可操作控制和调用前后状态。
- [ ] 补齐键盘、焦点、重复操作、非法值、空模型、RTL、长文本、125% 和 200% 自动或截图证据。
- [ ] 截图基线改用“分类/组件/场景”命名，不继续使用 P0/P1/P2 批次命名。

## 3. 平台增强

平台增强必须进入独立 crate，并为 Windows、macOS、Linux 保持统一公开语义；核心 `.slint` 组件不得直接调用系统 API。

- [ ] 窗口拖动与缩放。
- [ ] 原生窗口阴影。
- [ ] 系统托盘。
- [ ] 全局快捷键。
- [ ] 原生文件对话框。
- [ ] 系统通知。

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

## 6. 已知上游限制

- Tooltip 的延迟、偏移和通用焦点触发。
- Toolbar 任意子项的 roving focus、方向键遍历和自动溢出。
- 任意 `@children` 的运行时方向切换和自动换行。
- 任意业务组合的自动 RTL 结构镜像和方向图标镜像。
- 通用跨窗口焦点陷阱、所有者焦点恢复和平台模态关系。
- `ScrollArea` 虚拟化；超长数据集合继续使用 Table、DataGrid 或后续窗口数据协议。

## 7. 当前范围之外

FloatButton、Affix、Anchor、Carousel、Tour、Watermark、QRCode、Rate、Mentions、Transfer、Masonry、网页 24 栅格和 ProComponents 没有明确桌面产品需求。出现真实场景时先增加独立组件提案和规格，再决定是否实现。
