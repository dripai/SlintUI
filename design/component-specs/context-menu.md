# ContextMenu

状态：已实现（P1）

## 用途与边界
在对象或指针位置打开上下文命令；必须保留按钮、菜单或键盘等价入口。

## 公开 API
继承 PopupMenu 全部 API，新增 `anchor-x: length`、`anchor-y: length`。

## 状态与交互
打开、选择、关闭和键盘行为与 PopupMenu 一致；调用方在右键或 Shift+F10 时更新锚点并调用 `show()`。

## 无障碍与本地化
沿用 PopupMenu 语义；右键不能成为唯一入口，RTL 下锚点仍使用窗口坐标。

## Gallery、测试与限制
Gallery 与 PopupMenu 共用场景，编译和冒烟覆盖。多显示器工作区翻转由原生 PopupWindow 负责，焦点恢复目标由宿主记录。

遵循四份全局规范。
