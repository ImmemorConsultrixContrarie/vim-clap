" Author: liuchengxu <xuliuchengxlc@gmail.com>
" Description: Functions for working with the file path.

let s:save_cpo = &cpoptions
set cpoptions&vim

function! s:is_dir(pattern) abort
  return a:pattern[-1:] ==# '/'
endfunction

" Credit: vim-rooter
function! s:find_upwards(pattern) abort
  let start_dir = expand('#'.g:clap.start.bufnr.':p')
  let fd_dir = isdirectory(start_dir) ? start_dir : fnamemodify(start_dir, ':h')
  let fd_dir_escaped = escape(fd_dir, ' ')

  if s:is_dir(a:pattern)
    let match = finddir(a:pattern, fd_dir_escaped.';')
  else
    let [_suffixesadd, &suffixesadd] = [&suffixesadd, '']
    let match = findfile(a:pattern, fd_dir_escaped.';')
    let &suffixesadd = _suffixesadd
  endif

  if empty(match)
    return ''
  endif

  if s:is_dir(a:pattern)
    " If the directory we found (`match`) is part of the file's path
    " it is the project root and we return it.
    "
    " Compare with trailing path separators to avoid false positives.
    if stridx(fnamemodify(fd_dir, ':p'), fnamemodify(match, ':p')) == 0
      return fnamemodify(match, ':p:h')
    " Else the directory we found (`match`) is a subdirectory of the
    " project root, so return match's parent.
    else
      return fnamemodify(match, ':p:h:h')
    endif
  else
    return fnamemodify(match, ':p:h')
  endif
endfunction

" Find the nearest directory by searching upwards
" through the paths relative to the given buffer,
" given a bufnr and a directory name.
function! clap#path#find_nearest_dir(bufnr, dir) abort
  let fname = fnameescape(fnamemodify(bufname(a:bufnr), ':p'))

  let relative_path = finddir(a:dir, fname . ';')

  if !empty(relative_path)
    return fnamemodify(relative_path, ':p')
  endif

  return ''
endfunction

function! clap#path#get_git_root() abort
  let root = split(system('git rev-parse --show-toplevel'), '\n')[0]
  return v:shell_error ? '' : root
endfunction

" This is faster than clap#path#get_git_root() which uses the system call.
function! clap#path#find_git_root(bufnr) abort
  " git submodule uses .git instead of .git/. Ref #164
  for pattern in ['.git', '.git/']
    let git_dir = s:find_upwards(pattern)
    if !empty(git_dir)
      return git_dir
    endif
  endfor

  return ''
endfunction

function! clap#path#git_root_or_default(bufnr) abort
  let git_root = clap#path#find_git_root(a:bufnr)
  return empty(git_root) ? getcwd() : git_root
endfunction

let &cpoptions = s:save_cpo
unlet s:save_cpo
