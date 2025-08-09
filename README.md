# QuickPhish

QuickPhish is a small application to quickly analyse phishing emails. The app is developed with Rust to support all OS and is a local desktop app to avoid having another browser tab somewhere open which you can't find anymore.

## How it Works

Open a `.eml` file in **QuickPhish** and it will parse the email for you. It will write a quick summary for your case notes (using Jinja templates you can adjust yourself) and it will show you detailed information about the email, including:

- all headers extracted and displayed in table format
- all URLs and email addresses found in the email
- render of the email

![app screenshot](resources/app_screenshot_01.png)


## Frameworks

- Frontend with [TailwindCSS](https://tailwindcss.com)
- Icons from [HeroIcons](https://heroicons.com/outline)
- Backend with Rust & [Tauri](https://tauri.app/)