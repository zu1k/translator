# Copy Translator

Copy Translator 是使用Rust编写的翻译小工具

## 特点

- 轻量
- 自动去除多余的换行和断句，优化PDF翻译体验
- 选中即翻译(划词翻译)，专注论文阅读
- 使用DeepL进行翻译，翻译效果极佳，无需api token

## 使用说明

工具仅一个exe，启动后会驻留后台，选中文本后按 `ctrl+q` 唤起翻译界面，`esc`关闭界面，`ctrl+shift+d`完全退出

在界面开启的情况下，可以通过选中文本触发翻译行为，无需快捷键，俗称“划词翻译”

![使用截图](./res/pic.png)

### 搭建接口

- https://github.com/zu1k/deepl-api-rs
- https://hub.docker.com/r/zu1k/deepl

```sh
docker run -itd -p 8080:80 zu1k/deepl
```

## 感谢列表

- [DeepL](https://deepl.com/): DeepL翻译
- [LXGW WenKai](https://github.com/lxgw/LxgwWenKai): 霞鹜文楷字体
- [egui](https://github.com/emilk/egui): 本工具使用的Gui库

## Copyright

**Copy Translator** © [zu1k](https://github.com/zu1k), Copyright reserved.<br>

> Blog [zu1k.com](https://zu1k.com) · GitHub [@zu1k](https://github.com/zu1k) · Twitter [@zu1k_lv](https://twitter.com/zu1k_lv) · Telegram Channel [@peekfun](https://t.me/peekfun)
