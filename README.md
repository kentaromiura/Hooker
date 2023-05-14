# Hooker

Currently [https://learnjapanese.moe/vn/#playing-visual-novels-to-learn-japanese](The visual novel guide) to be able to read Japanese VNs with side-by-side text where one can use yomichan/10ten is good but I have a couple of issues with it:

- 1 it requires installing a browser plugin that can read the system clipboard, which I think is a security issue (although Chrome asks for it and requires user manual activation)

- 2 because of the plugin and Chrome security running the page `https://learnjapanese.moe/texthooker.html` locally (after saving it) would not work as `file://` protocol has higher protections; to make it work one needs either to host the file with a local server (very easy for tech people, less for non-tech and never explained in the guide) or disable all the protections which is a big security issue.

Because of 1, if the domain get squatted/dns get poisoned or someone has malicious intent and store some bad JS behind the texthooker page it can potentially leak sensitive informations such as username/passwords temporarly stored in the clipboard.

So because of the above I decided to make a one easy click program that reads the clipboard and pass it to the page the same way the chrome plugin does but without all the security issues, also the html page is embedded and served, allowing for offline usage.

The texthook page I suggest is https://anacreondjt.gitlab.io/docs/texthooker/ which explicitly allows to download for offline usage, I'm currently unsure if I can embed it into Hooker.

TODOs:

- make a status icon in the os bar to kill the process (https://github.com/olback/tray-item-rs/ ?)
- Using a single port and routing for the SSE part instead of using a different port

