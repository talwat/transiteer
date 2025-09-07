<script setup lang="ts">
import { computed, reactive, ref, useTemplateRef } from 'vue'
import { Color, Line, Point, TransitMap } from '/wasm'
import { onMounted, onBeforeUnmount } from 'vue'

const canvas = useTemplateRef('canvas')
var ctx: CanvasRenderingContext2D
onMounted(() => {
  window.addEventListener('resize', resize)
  resize()

  ctx = (canvas.value as HTMLCanvasElement).getContext('2d')!
})

onBeforeUnmount(() => {
  window.removeEventListener('resize', resize)
})

let data = TransitMap.new()
data.push_line(Line.new(Color.new(255, 0, 0), 'Red Line'))

const MAP_SIZE = 24

let svg = ref(data.svg())

let viewBox = ref([0, 0, MAP_SIZE, MAP_SIZE])
let viewBoxString = computed(() => viewBox.value.join(' '))

let held = false
let offset = [0, 0]
let initial = [0, 0]

const scale = ref()

function screenToWorld(x: number, y: number): [number, number] {
  const worldX = viewBox.value[0] * scale.value + x
  const worldY = viewBox.value[1] * scale.value + y

  const roundX = Math.round(worldX / scale.value)
  const roundY = Math.round(worldY / scale.value)
  return [roundX, roundY]
}

function resize() {
  if (canvas.value != null) {
    canvas.value.width = innerWidth
    canvas.value.height = innerHeight
  }

  scale.value = Math.min(window.innerWidth / MAP_SIZE, window.innerHeight / MAP_SIZE)
}

function move(event: MouseEvent) {
  if (held) {
    viewBox.value[0] = initial[0] + (offset[0] - event.clientX) / scale.value
    viewBox.value[1] = initial[1] + (offset[1] - event.clientY) / scale.value
  }

  const [worldX, worldY] = screenToWorld(event.clientX, event.clientY)

  const screenX = worldX * scale.value - viewBox.value[0] * scale.value
  const screenY = worldY * scale.value - viewBox.value[1] * scale.value

  ctx.reset()

  ctx.beginPath()
  ctx.arc(screenX, screenY, 8, 0, Math.PI * 2) // radius=3px
  ctx.fill()
}

function down(event: MouseEvent) {
  if (event.button != 2) return
  offset[0] = event.clientX
  offset[1] = event.clientY

  initial[0] = viewBox.value[0]
  initial[1] = viewBox.value[1]

  held = true
}

function up(event: MouseEvent) {
  if (event.button != 2) return
  held = false
}

function click(event: MouseEvent) {
  const [worldX, worldY] = screenToWorld(event.clientX, event.clientY)

  data.push_point(0, Point.new(worldX, worldY))
  svg.value = data.svg()
  console.log(worldX, worldY)
}
</script>

<template>
  <div
    id="view"
    @click="click"
    @mousemove="move"
    @mousedown="down"
    @mouseup="up"
    oncontextmenu="return false"
  >
    <div
      id="grid"
      :style="{
        backgroundSize: `${scale}px ${scale}px`,
        backgroundPositionX: `${viewBox[0] * -scale}px`,
        backgroundPositionY: `${viewBox[1] * -scale}px`,
      }"
    ></div>
    <svg
      ref="design"
      id="design"
      xmlns="http://www.w3.org/2000/svg"
      preserveAspectRatio="xMinYMin"
      :viewBox="viewBoxString"
      v-bind:innerHTML="svg"
    ></svg>
    <canvas id="canvas" ref="canvas"></canvas>
  </div>
</template>

<style scoped>
#view {
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  -webkit-user-select: none;
  -moz-user-select: none;
  user-select: none;
  z-index: 1;
}

#canvas {
  z-index: 2;
}

#view > * {
  width: 100%;
  height: 100%;
  top: 0;
  margin: 0;
  position: absolute;
  overflow: hidden;
}

#grid {
  background-image:
    linear-gradient(to right, grey 1px, transparent 1px),
    linear-gradient(to bottom, grey 1px, transparent 1px);
  z-index: 0;
}

#design {
  -webkit-user-select: none;
  -moz-user-select: none;
  user-select: none;
}
</style>
