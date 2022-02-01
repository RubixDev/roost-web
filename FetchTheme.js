const https = require('https')
const fs = require('fs')

const req = https.get('https://raw.githubusercontent.com/Binaryify/OneDark-Pro/master/themes/OneDark-Pro.json', res => {
    let body = ''

    res.on('data', chunk => {
        body += chunk
    })

    res.on('end', () => {
        body = JSON.parse(body)
        let out = {
            name: body.name,
            settings: [{
                settings: {
                    background: body.colors['editor.background'],
                    foreground: body.colors['editor.foreground'],
                }
            }]
        }
        out.settings.push(...body.tokenColors)

        fs.writeFileSync('web/src/one-dark-theme.ts', `export default ${JSON.stringify(out, null, 2)}\n`)
    })
})
req.end()
