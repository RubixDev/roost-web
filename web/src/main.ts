import { initMonaco } from './monaco'

main()

async function main() {
    const editor = await initMonaco()
    editor.onDidChangeModelContent(() => {
        localStorage.setItem('code', editor.getValue())
    })

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
}
