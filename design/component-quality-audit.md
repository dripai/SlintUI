# 组件质量复核记录

状态：已完成 2026-07-20 基线复核。

本记录对应 [`component-inventory.md`](component-inventory.md) 第 13 节完成定义。复核对象是扩展 P1 前已存在的 58 份规格；“通过”表示组件特有 API/Token 说明、适用状态、Gallery 示例及与风险匹配的自动或静态证据均已补齐，不表示三平台真机和外部业务接入已经完成。

证据缩写：`H0` 为原 P0/P1/P2 交互 Harness，`H1` 为本轮扩充的 P1 与既有边界 Harness，`G` 为 Gallery 单组件页，`S` 为组件规格，`C` 为编译/静态资源检查，`P` 为软件渲染截图。

## 基础、布局与操作（21）

| 组件 | 适用状态或变体 | 证据 | 结论 |
|---|---|---|---|
| Theme | light/dark/high-contrast、三密度、LTR/RTL、100%–200% | G/P/S | 通过 |
| Typography | 字阶、语义色、换行、长文本 | G/P/S | 通过 |
| Label | 默认、muted、required、换行 | G/S | 通过 |
| Icon | 12/16/20/24px、outlined/filled、装饰/命名 | G/C/P/S | 通过 |
| Surface | layout/surface/elevated、bordered | G/P/S | 通过 |
| Divider | 水平、垂直、subtle | G/S | 通过 |
| FocusRing | active、圆角、三主题 | G/P/S | 通过 |
| Overlay | active、dismissible、重复关闭 | G/H1/S | 通过 |
| ScrollArea | 长内容、滚动边界、缩放 | G/P/S | 通过 |
| Stack | gap、padding、长内容 | G/S | 通过 |
| Space | gap、alignment、长文本 | G/S | 通过 |
| Card | 标题、说明、长内容、bordered | G/P/S | 通过 |
| Button | 五变体、三尺寸、icon/loading/disabled/checkable | G/H0/P/S | 通过 |
| IconButton | normal、tooltip、disabled、loading | G/H0/S | 通过 |
| ToolButton | normal、checked、disabled | G/H0/S | 通过 |
| Toolbar | 图标/文字/分隔、焦点顺序、disabled | G/H0/S | 通过 |
| SplitPane | 水平/垂直、拖动、折叠、边界 | G/H0/P/S | 通过 |
| SettingsSection | 标题、说明、表单长文本 | G/S | 通过 |
| FormRow | required、help、error、长标签 | G/H0/S | 通过 |
| Form | valid/invalid、submitting、重复提交 | G/H0/S | 通过 |
| SplitButton | default/primary、loading、菜单边界 | G/H0/S | 通过 |

## 数据录入与导航（18）

| 组件 | 适用状态或变体 | 证据 | 结论 |
|---|---|---|---|
| Checkbox | unchecked/checked/indeterminate、disabled、focus | G/H0/S | 通过 |
| Switch | on/off、三尺寸、disabled、focus | G/H0/S | 通过 |
| TextField | empty/populated、clearable、read-only、disabled、validation | G/H0/P/S | 通过 |
| Select | empty/selected、disabled、边界选择 | G/H0/S | 通过 |
| SegmentedControl | selected、三尺寸、disabled、边界选择 | G/H0/S | 通过 |
| MultiSelect | empty/selected/overflow、disabled 请求 | G/H0/S | 通过 |
| AutoComplete | empty/populated/open、disabled、越界选择 | G/H0/S | 通过 |
| DatePicker | 有效日期、月末/闰年、disabled、越界 | G/H0/S | 通过 |
| TimePicker | 时分秒、边界、disabled | G/H0/S | 通过 |
| ColorPicker | selected、disabled、空选项 | G/H0/S | 通过 |
| Tabs | selected、closable、disabled、重复/越界选择 | G/H0/S | 通过 |
| PopupMenu | closed/open、checked、disabled、重复关闭 | G/H0/S | 通过 |
| ContextMenu | 指针打开、disabled、关闭协议 | G/H0/S | 通过 |
| Breadcrumb | 当前项、disabled、溢出边界 | G/H0/S | 通过 |
| Steps | waiting/current/complete/error、disabled | G/H0/S | 通过 |
| Pagination | 首尾页、页大小、重复/越界请求 | G/H0/S | 通过 |
| NavigationRail | selected、disabled、边界选择 | G/H0/S | 通过 |
| CommandPalette | open/query、disabled、重复/越界激活 | G/H0/S | 通过 |

## 数据展示与反馈（19）

| 组件 | 适用状态或变体 | 证据 | 结论 |
|---|---|---|---|
| Table | populated/empty、selected、排序请求、边界 | G/H1/S | 通过 |
| Tree | expanded/collapsed、selected、disabled、越界 | G/H0/S | 通过 |
| Avatar | image/initials/fallback、三尺寸 | G/S | 通过 |
| DataGrid | 窗口数据、empty、selected、编辑/越界请求 | G/H0/P/S | 通过 |
| Statistic | neutral/positive/warning/negative、长说明 | G/S | 通过 |
| Timeline | info/success/warning/error、pending、长内容 | G/S | 通过 |
| ImageView | loading/empty/error/image、重复 retry | G/H1/S | 通过 |
| Calendar | today/selected/outside/disabled、月切换边界 | G/H0/S | 通过 |
| EmptyState | no-data/no-result/no-permission、action slot | G/S | 通过 |
| Tooltip | enabled/disabled、短说明、可访问描述 | G/C/S | 通过 |
| Progress | determinate/indeterminate、边界、reduced-motion | G/P/S | 通过 |
| Spinner | 三尺寸、reduced-motion、命名 | G/S | 通过 |
| StatusBar | info/success/warning/error、busy、长详情 | G/S | 通过 |
| Toast | 四 tone、dismissible、禁用关闭、越界/重复关闭 | G/H1/S | 通过 |
| ModalDialog | accept/reject、busy、重复操作 | G/H1/S | 通过 |
| ConfirmDialog | normal/danger、busy、宿主协议 | G/H0/S | 通过 |
| Notification | 四 tone、action、busy、dismissible、重复操作 | G/H1/P/S | 通过 |
| Drawer | 四方向、dismissible、重复关闭 | G/H1/S | 通过 |
| Skeleton | avatar/rows、active、reduced-motion | G/S | 通过 |

合计：21 + 18 + 19 = 58 个组件。新增的 28 个 P1 组件使用同一规格模板、Gallery 页面和 `H1` 交互门禁，清单见 [`component-specs/`](component-specs/README.md)。颜色对比度的计算值见 [`contrast-audit.md`](contrast-audit.md)。

## 仍不在本结论内

- Windows GPU、系统 DPI、Narrator/NVDA，以及 macOS、Linux 真机矩阵。
- 原生窗口拖动、文件对话框、系统通知等平台增强。
- 首个真实业务设置页和工具栏接入。

这些项目继续保留在 [`../TODO.md`](../TODO.md)，不以 Gallery 或语义声明代替。
