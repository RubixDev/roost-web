import type { LanguageId } from './register'
import type { ScopeNameInfo } from './providers'

import * as monaco from 'monaco-editor/esm/vs/editor/editor.api'
import { createOnigScanner, createOnigString, loadWASM } from 'vscode-oniguruma'
import { SimpleLanguageInfoProvider } from './providers'
import { registerLanguages } from './register'
import { rehydrateRegexps } from './configuration'

import OneDarkTheme from './one-dark-theme'
import RoostConfig from './roost-config'
import RoostGrammar from './roost-grammar'

export async function initMonaco(): Promise<monaco.editor.IStandaloneCodeEditor> {
    const languages: monaco.languages.ILanguageExtensionPoint[] = [{
        id: 'roost',
        extensions: ['.ro'],
    }]
    const grammars: { [scopeName: string]: ScopeNameInfo } = {
        'source.roost': {
            language: 'roost'
        }
    }

    const data: ArrayBuffer | Response = await loadVSCodeOnigurumWASM()
    await loadWASM(data)
    const onigLib = Promise.resolve({
        createOnigScanner,
        createOnigString,
    })

    const provider = new SimpleLanguageInfoProvider({
        grammars,
        grammar: RoostGrammar,
        configurations: languages.map((language) => language.id),
        configuration: rehydrateRegexps(RoostConfig),
        theme: OneDarkTheme,
        onigLib,
        monaco,
    })
    registerLanguages(
        languages,
        (language: LanguageId) => provider.fetchLanguageInfo(language),
        monaco,
    )

    const element = document.getElementById('editor') as HTMLDivElement

    async function startCode(): Promise<string> {
        const savedCode = localStorage.getItem('code')
        if (savedCode === null) {
            const res = await fetch('examples/welcome.ro')
            if (!res.ok) return ''
            return await res.text()
        }
        return savedCode
    }

    const editor = monaco.editor.create(element, {
        value: await startCode(),
        language: 'roost',
        theme: 'vs-dark',
        minimap: {
            enabled: true,
        },
    })
    provider.injectCSS()
    window.addEventListener('resize', () => {
        editor.layout()
    })
    return editor
}

async function loadVSCodeOnigurumWASM(): Promise<Response | ArrayBuffer> {
    const response = await fetch('node_modules/vscode-oniguruma/release/onig.wasm')
    const contentType = response.headers.get('content-type')
    if (contentType === 'application/wasm') {
        return response
    }
    return await response.arrayBuffer()
}
