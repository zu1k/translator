# Copy Translator

Copy Translator 是使用Rust编写的翻译小工具

开发初衷是辅助论文阅读，因为不喜欢内存黑洞Electron，便自己开发一个轻量且简单的来替代

## 使用说明

工具仅一个exe，启动后会驻留后台，选中文本后按 `ctrl+d` 或 `ctrl+q` 唤起翻译界面，`esc`关闭界面，`ctrl+shift+d`完全退出

在界面开启的情况下，可以通过选中文本触发翻译行为，无需快捷键，俗称“划词翻译”

![使用截图](./res/pic.png)

## 版本说明

因为DeepL的jsonrpc接口有速率限制，所以提供下面两个版本：

- `copy-translator-online`: 使用我搭建的DeepL接口，利用腾讯云云函数，出口多个IP
- `copy-translator-local`: 使用内嵌在程序中的DeepL接口，使用本地的出口IP

请尽量使用local版本

## 感谢列表

- [CopyTranslator](https://copytranslator.github.io/): Electron版本CopyTranslator
- [DeepL](https://deepl.com/): DeepL翻译
- [LXGW WenKai](https://github.com/lxgw/LxgwWenKai): 霞鹜文楷字体
- [egui](https://github.com/emilk/egui): 本工具使用的Gui库
