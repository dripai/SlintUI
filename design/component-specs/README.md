# SlintUI 组件规格

本目录是公开组件契约的唯一来源。当前公开入口包含 88 个组件和 1 个 `Theme` 全局对象，共 89 份独立规格；每份规格对应一个公开名称，不再按 P0、P1、P2 或历史里程碑组织。

每份规格分别定义：

1. 公开 API：实际存在的属性、数据类型、内容入口、事件和公开方法。
2. 视觉规范：组成结构、变体、尺寸、状态外观、真实消费的 Theme Token 和环境表现。
3. 行为规范：状态所有权、输入与键盘、事件时序、方法、焦点、无障碍、本地化、边界和平台限制。

组件没有某类 API 时，文档直接省略该标题，不填写“无”或“不适用”。源码、Gallery、测试和本目录必须保持同一公开契约。所有规格共同遵循：

- [`foundations.md`](../foundations.md)
- [`interaction.md`](../interaction.md)
- [`accessibility.md`](../accessibility.md)
- [`content-and-localization.md`](../content-and-localization.md)
- [`gallery-information-architecture.md`](../gallery-information-architecture.md)

## 通用 General（14）

- [`Theme`](general/theme.md)
- [`Typography`](general/typography.md)
- [`Label`](general/label.md)
- [`Icon`](general/icon.md)
- [`Surface`](general/surface.md)
- [`FocusRing`](general/focus-ring.md)
- [`Overlay`](general/overlay.md)
- [`Button`](general/button.md)
- [`ButtonGroup`](general/button-group.md)
- [`IconButton`](general/icon-button.md)
- [`ToolButton`](general/tool-button.md)
- [`SplitButton`](general/split-button.md)
- [`ToggleButton`](general/toggle-button.md)
- [`Link`](general/link.md)

## 布局 Layout（9）

- [`Divider`](layout/divider.md)
- [`ScrollArea`](layout/scroll-area.md)
- [`Stack`](layout/stack.md)
- [`Space`](layout/space.md)
- [`Flex`](layout/flex.md)
- [`Grid`](layout/grid.md)
- [`Panel`](layout/panel.md)
- [`SplitPane`](layout/split-pane.md)
- [`Toolbar`](layout/toolbar.md)

## 导航 Navigation（9）

- [`Breadcrumb`](navigation/breadcrumb.md)
- [`ContextMenu`](navigation/context-menu.md)
- [`Dropdown`](navigation/dropdown.md)
- [`NavigationRail`](navigation/navigation-rail.md)
- [`Pagination`](navigation/pagination.md)
- [`PopupMenu`](navigation/popup-menu.md)
- [`PopupMenuItem`](navigation/popup-menu-item.md)
- [`Steps`](navigation/steps.md)
- [`Tabs`](navigation/tabs.md)

## 数据录入 Data Entry（21）

- [`AutoComplete`](data-entry/auto-complete.md)
- [`Checkbox`](data-entry/checkbox.md)
- [`CheckboxGroup`](data-entry/checkbox-group.md)
- [`ColorPicker`](data-entry/color-picker.md)
- [`ComboBox`](data-entry/combo-box.md)
- [`DatePicker`](data-entry/date-picker.md)
- [`FilePicker`](data-entry/file-picker.md)
- [`Form`](data-entry/form.md)
- [`FormRow`](data-entry/form-row.md)
- [`MultiSelect`](data-entry/multi-select.md)
- [`NumberInput`](data-entry/number-input.md)
- [`Radio`](data-entry/radio.md)
- [`RadioGroup`](data-entry/radio-group.md)
- [`SearchField`](data-entry/search-field.md)
- [`SegmentedControl`](data-entry/segmented-control.md)
- [`Select`](data-entry/select.md)
- [`Slider`](data-entry/slider.md)
- [`Switch`](data-entry/switch.md)
- [`TextArea`](data-entry/text-area.md)
- [`TextField`](data-entry/text-field.md)
- [`TimePicker`](data-entry/time-picker.md)

## 数据展示 Data Display（18）

- [`Avatar`](data-display/avatar.md)
- [`Badge`](data-display/badge.md)
- [`Calendar`](data-display/calendar.md)
- [`Card`](data-display/card.md)
- [`Collapse`](data-display/collapse.md)
- [`DataGrid`](data-display/data-grid.md)
- [`DescriptionList`](data-display/description-list.md)
- [`EmptyState`](data-display/empty-state.md)
- [`ImageView`](data-display/image-view.md)
- [`List`](data-display/list.md)
- [`ListItem`](data-display/list-item.md)
- [`Popover`](data-display/popover.md)
- [`Statistic`](data-display/statistic.md)
- [`Table`](data-display/table.md)
- [`Tag`](data-display/tag.md)
- [`Timeline`](data-display/timeline.md)
- [`Tooltip`](data-display/tooltip.md)
- [`Tree`](data-display/tree.md)

## 反馈 Feedback（12）

- [`Alert`](feedback/alert.md)
- [`ConfirmDialog`](feedback/confirm-dialog.md)
- [`Drawer`](feedback/drawer.md)
- [`LoadingOverlay`](feedback/loading-overlay.md)
- [`ModalDialog`](feedback/modal-dialog.md)
- [`Notification`](feedback/notification.md)
- [`Progress`](feedback/progress.md)
- [`ProgressRing`](feedback/progress-ring.md)
- [`ResultState`](feedback/result-state.md)
- [`Skeleton`](feedback/skeleton.md)
- [`Spinner`](feedback/spinner.md)
- [`Toast`](feedback/toast.md)

## 桌面 Desktop（6）

- [`AppShell`](desktop/app-shell.md)
- [`CommandPalette`](desktop/command-palette.md)
- [`SettingsSection`](desktop/settings-section.md)
- [`ShortcutHint`](desktop/shortcut-hint.md)
- [`StatusBar`](desktop/status-bar.md)
- [`TitleBar`](desktop/title-bar.md)

图标 SVG 资源不按单个组件维护规格；资源命名、许可、按需导入和 RTL 规则见 [`docs/iconography.md`](../../docs/iconography.md)。尚未完成的平台验证、真实产品接入和发布任务见 [`TODO.md`](../../TODO.md)。
