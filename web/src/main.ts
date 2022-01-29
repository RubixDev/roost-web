import { initMonaco } from './monaco'

const examples = [
    {
        name: 'Welcome',
        file: 'welcome.ro',
    },
    {
        name: 'Fibonacci',
        file: 'fib.ro',
    },
]

main()

async function main() {
    const editor = await initMonaco()
    editor.onDidChangeModelContent(() => {
        localStorage.setItem('code', editor.getValue())
    })

    // --------------- Web Worker ----------------
    let roostWorker: Worker | null = null;

    const runButton = document.getElementById('run-button') as HTMLButtonElement
    const killButton = document.getElementById('kill-button') as HTMLButtonElement
    const consoleDiv = document.getElementById('console') as HTMLDivElement

    runButton.addEventListener('click', () => {
        consoleDiv.innerHTML = ''
        roostWorker?.terminate()
        roostWorker = makeWorker(editor.getValue())
        killButton.disabled = false
    })

    killButton.addEventListener('click', () => {
        roostWorker?.terminate()
        killButton.disabled = true
    })

    function makeWorker(code: string): Worker {
        let worker = new Worker(new URL('./roost.worker.ts', import.meta.url))
        worker.onmessage = function (event: { data: any[] }) {
            if (event.data[0] === 'print') {
                consoleDiv.innerHTML += event.data[1]
                consoleDiv.scrollTo(0, consoleDiv.scrollHeight)
            }
            if (event.data[0] === 'ready') worker.postMessage(['run', code])
            if (event.data[0] === 'finished') {
                roostWorker?.terminate()
                killButton.disabled = true
            }
        }
        return worker
    }

    // -------------- Example Selection --------------
    const loadSelect = document.getElementById('load-select') as HTMLLabelElement
    const loadSelectInput = document.getElementById('load-select-cb') as HTMLInputElement
    const loadSelectError = document.getElementById('load-select-error') as HTMLSpanElement
    const currentOption = document.getElementById('load-select__current') as HTMLDivElement
    const loadButton = document.getElementById('load-button') as HTMLButtonElement
    for (const example of examples) {
        const exampleOption = document.createElement('div')
        exampleOption.className = 'load-select-option'
        exampleOption.innerHTML = `<span>${example.name}</span>`
        loadSelect.appendChild(exampleOption)
        exampleOption.addEventListener('click', () => {
            currentOption.innerHTML = exampleOption.innerHTML
            currentOption.setAttribute('data-value', example.file)
            loadButton.disabled = false
        })
    }

    let mouseOverSelect = false
    loadSelect.onmouseover = () => mouseOverSelect = true
    loadSelect.onmouseout  = () => mouseOverSelect = false
    window.addEventListener('mousedown', () => {
        if (loadSelectInput.checked && !mouseOverSelect)
            loadSelectInput.checked = false
    })

    loadButton.addEventListener('click', async () => {
        loadButton.disabled = true
        const res = await fetch(`examples/${currentOption.getAttribute('data-value')}`)
        if (!res.ok) {
            loadButton.disabled = false
            loadSelectError.style.display = 'block'

            loadButton.style.backgroundColor = 'var(--red)'
            loadButton.style.color = 'var(--bg-light)'
            setTimeout(() => {
                loadButton.style.backgroundColor = ''
                loadButton.style.color = ''
                loadButton.style.transitionDuration = '500ms'
                setTimeout(() => loadButton.style.transitionDuration = '', 500)
            }, 200)

            return
        }
        const code = await res.text()
        editor.setValue(code)
        loadButton.disabled = false
        loadSelectError.style.display = 'none'
    })
}
