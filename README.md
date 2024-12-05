# Cursor-Mate

Cursor-Mate 是一个用于管理 Cursor 配置文件的命令行工具。
解决Cursor删除账号再登陆后，出现提示“Too many free trial accounts used on this machine.”的问题。

## 功能特点

- 查看机器 ID 信息
- 生成随机机器 ID
- 删除机器 ID 文件
- 终止 Cursor 进程
- 跨平台支持

## 系统支持

- Windows (x64)
- macOS (Intel x64)
- macOS (Apple Silicon)

## 安装

[Releases](https://github.com/korykim/cursormate/releases)


## 使用方法

cursor-mate
```

cursor-mate.exe <命令>

### 可用命令

| 命令 | 说明 |
|------|------|
| `ids` | 显示机器 ID 信息 |
| `random-ids` | 生成随机机器 ID |
| `delete` | 删除机器 ID 文件 |
| `kill` | 终止所有 Cursor 进程 |
| `help` | 显示帮助信息 |

### 选项

- `-h, --help`: 显示帮助信息

## 示例

显示机器 ID：

```bash
cursor-mate.exe ids
```

生成随机机器 ID：

```bash
cursor-mate.exe random-ids
```

删除机器 ID 文件：

```bash
cursor-mate.exe delete
```

终止 Cursor 进程：

```bash
cursor-mate.exe kill
```

## 许可证

[MIT](LICENSE)

 
