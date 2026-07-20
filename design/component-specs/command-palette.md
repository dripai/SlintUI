# CommandPalette

状态：已实现（P2）

## 用途与边界
展示可搜索命令列表；命令注册、排序、最近使用和快捷键监听属于宿主。

## 公开 API
`commands: [CommandItem]`、`query`、`current-index`、`open`、名称和占位文案；`query-changed`、`selected`、`close-requested`；`activate(index)`。

## 状态与交互
覆盖可见、选中、Hover、Disabled 和 Escape 关闭请求；筛选结果由 `visible` 控制。

## 无障碍与本地化
根为 groupbox，命令为 list-item，说明与快捷键由调用方本地化和格式化。

## Gallery、测试与限制
Gallery 提供打开入口；Harness 验证启用/禁用。当前不内建模糊搜索、全局快捷键、最近命令存储和焦点恢复。

遵循四份全局规范。
