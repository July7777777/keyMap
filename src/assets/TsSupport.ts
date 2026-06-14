// 定义技能类型
export interface Skill {
  name: string
  key: string
  gif: string
  delay: string
}

export interface SkillCategory {
  [skillName: string]: Skill
}