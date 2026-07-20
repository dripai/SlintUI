# 组件 API

公开入口：`@slint-ui/index.slint`。

- 全局与基础：Theme、Typography、Label、Icon、Surface、Divider、FocusRing、Overlay、ShortcutHint
- 布局与容器：ScrollArea、Stack、Space、Card、Toolbar、SplitPane、FormRow、Form、SettingsSection、Flex、Grid、Panel、AppShell、TitleBar
- 操作：Button、IconButton、ToolButton、SplitButton、ButtonGroup、ToggleButton、Link
- 导航：Tabs、PopupMenu、ContextMenu、Dropdown、Breadcrumb、Steps、Pagination、NavigationRail、CommandPalette
- 数据录入：Checkbox、Switch、TextField、TextArea、SearchField、NumberInput、Select、SegmentedControl、CheckboxGroup、Radio、RadioGroup、ComboBox、Slider、FilePicker、MultiSelect、AutoComplete、DatePicker、TimePicker、ColorPicker
- 数据展示：Table、Tree、Avatar、DataGrid、Statistic、Timeline、ImageView、Calendar、Badge、Tag、List、DescriptionList、Collapse、Popover
- 反馈：Tooltip、Progress、ProgressRing、Spinner、EmptyState、ResultState、StatusBar、Toast、Alert、Notification、Drawer、LoadingOverlay、Skeleton、ModalDialog、ConfirmDialog

图标资源不从主入口导出。按需导入与完整清单见 [`../iconography.md`](../iconography.md)。

以上 86 份组件规格对应的公开类型均已从入口导出（组合项如 `Radio` / `RadioGroup` 共用一份规格）。逐组件 API、状态、原生复用和限制见 [`../../design/component-specs/`](../../design/component-specs/README.md)；平台与产品验证任务见 [`../../TODO.md`](../../TODO.md)。
