# StatusBar

状态：已实现。源码：`ui/feedback/status-bar.slint`。

## 用途与边界

用于窗口级状态、连接信息和简短进度说明。需要用户操作的重要错误使用 Alert/Dialog，短暂通知使用后续 Toast；StatusBar 不保存历史。

## 公开 API

`text: string = ""`、`detail: string = ""`、`tone: StatusTone = neutral`（neutral/info/success/warning/error）、`busy: bool = false`、`accessible-name: string = text`。

## 状态与交互

支持五种 tone 和 busy；不接收指针、键盘或焦点。busy 显示 Spinner，否则显示状态点；文字和颜色共同表达状态，长内容省略。

## 无障碍与本地化

角色为 content-info，名称、说明和 polite live region 用于状态更新。调用方提供已本地化的短文本；状态色不能作为唯一信息，RTL 下文本顺序由内容语义决定。

## Gallery、测试与限制

Gallery 全局底栏及“反馈”页覆盖 info/success、Locale、方向和缩放信息；通过编译、冒烟和截图验收。没有可变回调，因而不需要交互状态转换测试。遵循四份全局规范。
