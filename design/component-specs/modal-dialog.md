# ModalDialog

状态：已实现（P1）

## 用途与边界
独立顶层阻塞决策窗口；普通信息使用页面内反馈，系统文件选择使用平台增强接口。

## 公开 API
标题、message、accept/cancel text、danger、busy、close-on-escape、accessible-name；`accepted/rejected`；`accept()`、`reject()`，并支持内容 children。

## 状态与交互
busy 阻止关闭与重复提交；Esc 按策略拒绝，危险操作使用 danger 样式。窗口内 Tab 顺序由 Slint 焦点系统管理。

## 无障碍与本地化
内容为 groupbox，提供名称和描述；按钮文案由宿主传入，危险确认默认焦点策略由宿主窗口创建流程决定。

## Gallery、测试与限制
Gallery 记录宿主打开方式，顶层组件由编译与冒烟覆盖。Slint 1.17.1 核心层不公开跨窗口 owner/modal API，宿主负责生命周期、焦点恢复和多窗口主题同步。

遵循四份全局规范。
