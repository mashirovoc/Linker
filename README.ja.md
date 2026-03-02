# Linker

CLIジャンプツール — パス、URL、ファイルを短い名前で登録し、瞬時にアクセス（移動・起動）できます。

```bash
l add docs ~/Documents/projects/docs
l docs          # → ~/Documents/projects/docs へ移動 (cd)

l add gh https://github.com
l gh            # → ブラウザで開く
```

## インストール

### Windows

**[Releases ページ](https://github.com/mashirovoc/Linker/releases/latest)** からインストーラーをダウンロードして実行してください。

インストーラーは以下の処理を行います：

- `linker` をシステム環境変数 PATH に追加
- （任意）`l` シェル関数を PowerShell プロファイル（PS5 および PS7）に追加

### Linux / macOS / WSL

```sh
curl -fsSL https://raw.githubusercontent.com/mashirovoc/Linker/main/installer/install.sh | bash
```

実行後、シェルを再読み込みしてください：

```sh
source ~/.bashrc   # bash の場合
source ~/.zshrc    # zsh の場合
```

### ソースからビルド

```sh
cargo build --release
# バイナリを PATH に追加した後、シェル関数の設定を行ってください：
linker init --shell bash        # bash 用のスニペットを表示 → ~/.bashrc に追記
linker init --shell powershell  # PowerShell 用のスニペットを表示 → $PROFILE に追記
```

## 使い方

すべてのコマンドは、シェルラッパーの `l`（または直接 `linker`）を介して呼び出します。

### エントリを追加する

```sh
l add <名前> <ターゲット>
```

`<ターゲット>` には以下を指定できます：

- **ディレクトリパス** — `l <名前>` でそのディレクトリに `cd` します
- **URL** (`http://`, `https://`, `ftp://`) — デフォルトのブラウザで開きます
- **ファイルパス** — デフォルトのアプリケーションで開きます
- **実行ファイル** (Windows の場合は `.exe`, `.bat`, `.cmd`) — そのまま実行します

### エントリにジャンプする

```sh
l <名前>
```

マッチングは以下の優先順位で行われます：

1. **完全一致** (Exact match)
2. **前方一致** (Prefix match)
3. **部分一致** (Substring match)

同じ優先度で複数のエントリがマッチした場合は、候補がリスト表示され、アクションは実行されません。

### エントリを表示する

```sh
l list
```

### エントリを編集する

```sh
l edit <名前> <新しいターゲット>
```

### エントリを削除する

```sh
l remove <名前>
```

## 設定

設定ファイルの保存場所：

- Windows: `%AppData%\linker\config.toml`
- Linux/macOS: `~/.config/linker/config.toml`

```toml
[opener]
url  = "default"   # URLを開くコマンド (例: "chrome")
file = "default"   # ファイルを開くコマンド (例: "code")
app  = "default"   # アプリを起動するコマンド
```

`"default"` を指定すると、OS標準のハンドラ（Windowsなら `start`、macOSなら `open`、Linuxなら `xdg-open`）が使用されます。

また、`config.toml` と同じディレクトリにある `bookmarks.json` を直接編集することで、エントリごとに `open_with` を指定してランチャーを上書きすることも可能です。

## ライセンス

MIT
