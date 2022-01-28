import { initMonaco } from './monaco'

main()

async function main() {
    const editor = await initMonaco()

    let roostWorker: Worker | null = null;

    const runButton = document.getElementById('button') as HTMLButtonElement
    const killButton = document.getElementById('button2') as HTMLButtonElement

    runButton.addEventListener('click', () => {
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
            if (event.data[0] === 'print') console.log(event.data[1])
            if (event.data[0] === 'ready') worker.postMessage(['run', code])
            if (event.data[0] === 'finished') {
                roostWorker?.terminate()
                killButton.disabled = true
            }
        }
        return worker
    }
}
