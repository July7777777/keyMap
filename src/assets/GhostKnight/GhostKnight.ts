import { Skill} from '../TsSupport'
import { AsuraTrigger } from './trigger'

const parseFilename = (filename: string): { name: string; delay: string } => {
  const basename = filename.split('/').pop()?.replace(/\.gif$/i, '') || ''
  const [skillName, delay] = basename.split('-')
  return { name: skillName, delay: delay || '2' }
}

const createSkillMap = (
  globResult: Record<string, { default: string }>
): Record<string, { gif: string; delay: string }> => {
  const map: Record<string, { gif: string; delay: string }> = {}
  for (const [path, module] of Object.entries(globResult)) {
    const { name, delay } = parseFilename(path)
    map[name] = { gif: module.default, delay }
  }
  return map
}

const commonGlob = import.meta.glob('./通用/*.gif', { eager: true }) as Record<string, { default: string }>
const asuraGlob = import.meta.glob('./阿修罗/*.gif', { eager: true }) as Record<string, { default: string }>

const commonSkillMap = createSkillMap(commonGlob)
const asuraSkillMap = createSkillMap(asuraGlob)

const buildKeyToSkill = (
  keyMap: Record<string, string>,
  skillMap: Record<string, { gif: string; delay: string }>,
  fallbackSkillMap: Record<string, { gif: string; delay: string }>
): Record<string, Skill> => {
  const result: Record<string, Skill> = {}
  for (const [key, skillName] of Object.entries(keyMap)) {
    const skillData = skillMap[skillName] || fallbackSkillMap[skillName]
    if (skillData) {
      result[key] = {
        name: skillName,
        key,
        gif: skillData.gif,
        delay: skillData.delay,
      }
    }
  }
  return result
}

const Asura = buildKeyToSkill(AsuraTrigger, asuraSkillMap, commonSkillMap)

export default {
  Asura,
}