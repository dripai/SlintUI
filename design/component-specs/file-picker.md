# FilePicker

状态：已实现（P1）。源码：`ui/controls/file-picker.slint`。

## 用途与边界
提供路径展示和浏览入口；原生文件对话框属于宿主或平台增强，不在核心组件中静默实现。

## 公开 API
`path`、`placeholder-text`、`browse-text`、`enabled`、`size`、`accessible-name`；`request-browse()`、`choose(path)`、`browse-requested()`、`changed()`。

## 状态与交互
覆盖 Empty、Populated、Focused button、Disabled、长路径和程序化选择；同路径不重复 changed，禁用时不请求或更新。

## 无障碍与本地化
根使用 groupbox，内部只读 TextField 与 Button 保持清晰焦点；浏览文案和名称必须本地化。

## Gallery、测试与限制
Gallery 展示路径和浏览入口；Harness 覆盖选择、重复、浏览请求和禁用。过滤器、权限、取消与路径规范化由宿主负责。

遵循四份全局规范。
