# 组件 API

公开入口：`@slint-ui/index.slint`。

- 全局与基础：Theme、Typography、Label、Icon、Surface、Divider、FocusRing、Overlay
- 布局与容器：ScrollArea、Stack、Space、Card、Toolbar、SplitPane、FormRow、Form、SettingsSection
- 操作：Button、IconButton、ToolButton、SplitButton
- 导航：Tabs、PopupMenu、ContextMenu、Breadcrumb、Steps、Pagination、NavigationRail、CommandPalette
- 数据录入：Checkbox、Switch、TextField、Select、SegmentedControl、MultiSelect、AutoComplete、DatePicker、TimePicker、ColorPicker
- 数据展示：Table、Tree、Avatar、DataGrid、Statistic、Timeline、ImageView、Calendar
- 反馈：Tooltip、Progress、Spinner、EmptyState、StatusBar、Toast、Notification、Drawer、Skeleton、ModalDialog、ConfirmDialog

图标资源不从主入口导出。按需导入与完整清单见 [`../iconography.md`](../iconography.md)。

以上 58 个组件均已从公开入口导出。逐组件 API、状态、原生复用和限制见 [`../../design/component-specs/`](../../design/component-specs/README.md)；未完成的质量与扩展任务见 [`../../TODO.md`](../../TODO.md)。
