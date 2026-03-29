#!/usr/bin/env node
/**
 * Merge split manual chapters into single-file help resources.
 *
 * Source:  doc/manual/{zh-CN,en-US}/*.md
 * Output:  src-tauri/resources/help/manual-{zh-CN,en-US}.md
 *
 * Usage: node scripts/merge-manual.mjs
 */

import { readFileSync, writeFileSync, readdirSync, existsSync, mkdirSync } from 'fs'
import { join, dirname } from 'path'
import { fileURLToPath } from 'url'

const __dirname = dirname(fileURLToPath(import.meta.url))
const projectRoot = join(__dirname, '..')

/**
 * Convert cross-file links to same-file anchor links.
 * Examples:
 *   (01-introduction.md)           -> remove link, keep text
 *   (01-introduction.md#11-purpose) -> (#11-purpose)
 *   (14-settings.md#144-shortcut-settings) -> (#144-shortcut-settings)
 *   (appendix-changelog.md)        -> remove link, keep text
 *   (appendix-changelog.md#xxx)    -> (#xxx)
 */
function convertLinks(content) {
  // Links with anchors: keep the anchor
  content = content.replace(
    /\[([^\]]*)\]\(\d+-[a-z-]+\.md(#[^)]+)\)/g,
    '[$1]($2)',
  )
  // Links without anchors: just keep the text (no link)
  content = content.replace(/\[([^\]]*)\]\(\d+-[a-z-]+\.md\)/g, '$1')

  // Same for appendix files
  content = content.replace(
    /\[([^\]]*)\]\(appendix-[a-z-]+\.md(#[^)]+)\)/g,
    '[$1]($2)',
  )
  content = content.replace(/\[([^\]]*)\]\(appendix-[a-z-]+\.md\)/g, '$1')

  return content
}

function mergeManual(lang) {
  const srcDir = join(projectRoot, 'doc', 'manual', lang)
  const output = join(projectRoot, 'src-tauri', 'resources', 'help', `manual-${lang}.md`)

  if (!existsSync(srcDir)) {
    console.warn(`Warning: ${srcDir} not found, skipping ${lang}`)
    return
  }

  console.log(`Merging ${lang} manual...`)

  const parts = []

  // 1. Index file (title + TOC)
  const indexContent = readFileSync(join(srcDir, 'index.md'), 'utf-8')
  parts.push(convertLinks(indexContent))

  // 2. Numbered chapter files in order
  const chapterFiles = readdirSync(srcDir)
    .filter((f) => /^\d+-.*\.md$/.test(f))
    .sort()

  for (const file of chapterFiles) {
    const content = readFileSync(join(srcDir, file), 'utf-8')
    parts.push('---\n')
    parts.push(convertLinks(content))
  }

  // 3. Appendix changelog
  const appendixPath = join(srcDir, 'appendix-changelog.md')
  if (existsSync(appendixPath)) {
    const content = readFileSync(appendixPath, 'utf-8')
    parts.push('---\n')
    parts.push(convertLinks(content))
  }

  const merged = parts.join('\n')
  writeFileSync(output, merged, 'utf-8')

  const lines = merged.split('\n').length
  console.log(`  -> ${output} (${lines} lines)`)
}

// Ensure output directory exists
const outputDir = join(projectRoot, 'src-tauri', 'resources', 'help')
if (!existsSync(outputDir)) {
  mkdirSync(outputDir, { recursive: true })
}

mergeManual('zh-CN')
mergeManual('en-US')

console.log('Manual merge complete.')
