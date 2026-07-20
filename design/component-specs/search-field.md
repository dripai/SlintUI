# SearchField

状态：已实现（P1）。源码：`ui/controls/search-field.slint`。

## 用途与边界
用于输入并提交查询；普通文本使用 TextField，候选建议使用 AutoComplete。

## 公开 API
继承 TextField 的属性和回调，固定搜索前缀图标与 clearable；新增 `submit()`、`searched(query)`。

## 状态与交互
覆盖 Default、Focused、Populated、Empty、Clear、Disabled、Validation 和长文本；Enter 与 `submit()` 触发搜索，连续提交保留显式用户意图。

## 无障碍与本地化
继承 text-input 语义；`accessible-name`、placeholder 和清除按钮名称必须本地化。

## Gallery、测试与限制
Gallery 展示有值和禁用；Harness 覆盖重复提交。延迟输入由宿主去抖，不内建计时和数据请求。

遵循四份全局规范。
