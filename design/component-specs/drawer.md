# Drawer

状态：已实现（P2）

## 用途与边界
在当前窗口侧边承载辅助任务；必须打断流程时使用 ModalDialog。

## 公开 API
`open`、`side: DrawerSide`、`panel-width`、标题/名称、遮罩与 Escape 策略；`close-requested()`；默认内容插槽。

## 状态与交互
支持 start/end 与 RTL 映射、遮罩关闭和 Escape 关闭请求；开关状态由宿主控制。

## 无障碍与本地化
遮罩区域和面板分别提供 region/groupbox 语义；标题与关闭名称由调用方本地化。

## Gallery、测试与限制
Gallery 提供打开入口。当前稳定 Slint 公开能力不提供通用焦点陷阱、所有者焦点恢复和平台级模态关系；宿主负责打开前后焦点。

遵循四份全局规范。
