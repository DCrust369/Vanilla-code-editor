project('meu_editor', 'c', 'rust',
  version: '0.1.0',
  default_options: ['warning_level=3', 'rust_std=2021']
)

# 1. Compilação do arquivo escrito em C (ex: config.c / git.c / salve_files.c)
# Ajuste os nomes dos fontes conforme a sua listagem de arquivos
c_sources = [
  'config.c',
  'git.c',
  'salve_files.c'
]

c_lib = static_library('c_helpers', c_sources)

# 2. Compilação do código principal em Rust (ex: vanilla.rs / kilo.rs)
# O Meson permite linkar a biblioteca C diretamente ao alvo Rust ou vice-versa
executable('meu_editor_exec',
  'vanilla.rs',
  link_with: c_lib,
  install: true
)

# 3. Integração opcional do Forth (gforth) no fluxo de build ou teste
gforth = find_program('gforth', required: false)
if gforth.found()
  custom_target('rodar_forth',
    output: 'forth_checked.stamp',
    input: 'true_false.fs',
    command: [gforth, '@INPUT@', '-e', 'bye', '&&', 'touch', '@OUTPUT@']
  )
endif
