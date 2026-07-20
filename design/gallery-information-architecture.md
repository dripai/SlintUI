# Gallery 信息架构与展示规范

Gallery 面向组件使用者和设计评审，不展示 P0、P1、P2、里程碑或开发批次。这些信息只属于 [`component-inventory.md`](component-inventory.md) 的实施管理。

## 浏览结构

左侧固定采用两层导航：

1. 一级沿用 Ant Design 的 General、Layout、Navigation、Data Entry、Data Display、Feedback 分类，中文显示为通用、布局、导航、数据录入、数据展示、反馈；SlintUI 额外保留图标资源入口，Theme 放在通用分类首项。
2. 二级直接对应单个公开组件，不设置“桌面 P1”“进阶 P2”或含义重叠的组件组。
3. 同一时刻最多展开一个一级分类；再次点击已展开分类时收起，点击其他分类时展开并默认选择该分类首个组件。收起导航不得清空或切换右侧当前组件。
4. 导航区域独立滚动，不能挤压右侧内容，也不能依赖窗口整体滚动寻找组件。
5. 右侧详情区域独立提供纵向滚动；缩放后内容超过可视宽度时同时提供横向滚动，API、Theme Token 和限制说明必须始终可达。

顶部标题单独占一行；全局主题、密度、缩放和语言放在同一控制行。语言选择仅显示简短语言名称，Locale 代码保留在状态栏中。

## 组件详情页

右侧一次只展示一个组件，并使用统一顺序：

1. 组件名称、一句用途说明和“何时使用”。
2. Examples，由多个独立示例卡片组成；每张卡片只验证一种主题，例如类型、图标、尺寸、禁用、加载或受控数据。
3. API 摘要。
4. 相关 Theme Token。
5. 宿主职责、已知限制和 FAQ。

示例画布统一使用 840px 最大内容宽度、Theme 间距和 Card 表面。禁止在同一画布中堆入不同任务的组件，也禁止为了填满页面随意改变组件宽度。

## 分类边界

- 通用：Theme、Typography、Label、Icon、Button、ButtonGroup、ToggleButton、Link 和 ShortcutHint。
- 布局：Surface、Divider、FocusRing、Stack、Space、ScrollArea、Overlay、SplitPane、Toolbar、设置容器、Flex、Grid、Panel、AppShell 和 TitleBar。
- 导航：Tabs、菜单、Dropdown，以及改变当前位置、层级、步骤或命令上下文的组件。
- 数据录入：Form、FormRow，以及产生或编辑业务值的文本、数值、选择、日期时间、颜色和文件入口组件。
- 数据展示：Card、SegmentedControl、Tooltip、EmptyState、Badge、Tag、List、DescriptionList、Collapse、Popover，以及只读呈现结构化数据或媒体的组件。
- 反馈：Progress、ProgressRing、Spinner、StatusBar、Toast、Alert、Dialog、Notification、Drawer、ResultState、LoadingOverlay 和 Skeleton。
- 图标：按 Outlined、Filled 资源目录浏览。

组件的实施优先级变化不得改变 Gallery 分类或 URL/选择语义。

## 验收

- 中文、英文、阿拉伯语 RTL 下分类和组件名称可读。
- 浅色、深色、高对比度及 100%–200% 缩放下，左侧导航与右侧画布互不遮挡。
- 每个二级条目只打开对应组件详情，不显示同批次其他组件。
- 截图基线按“分类 + 组件”命名，不再按 P0/P1/P2 命名。
