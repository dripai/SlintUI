# Timeline

状态：已实现（P2）

## 用途与边界
按顺序展示事件节点和状态，不用于精确时间轴编辑。

## 公开 API
`items: [TimelineItem]`、`accessible-name`；`selected(index)`；`select(index)`；`TimelineTone`。

## 状态与交互
覆盖五种语义色、Hover 和 Disabled；条目由宿主按时间顺序提供。

## 无障碍与本地化
根为 list，条目朗读标题、详情和时间；日期时间字符串由宿主本地化。

## Gallery、测试与限制
Gallery 展示两种状态；公开选择方法经编译覆盖。当前不内建折叠、异步加载和左右交错布局。

遵循四份全局规范。
