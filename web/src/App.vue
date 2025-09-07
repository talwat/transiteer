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
let cursor = ref([0, 0])

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

function worldToScreen(x: number, y: number): [number, number] {
  const screenX = x * scale.value - viewBox.value[0] * scale.value
  const screenY = y * scale.value - viewBox.value[1] * scale.value

  return [screenX, screenY]
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
  cursor.value[0] = worldX
  cursor.value[1] = worldY

  ctx.reset()

  const lastPoint = data.latest_point(0)
  if (lastPoint != null) {
    const [sLastX, sLastY] = worldToScreen(lastPoint.x, lastPoint.y)
    ctx.moveTo(sLastX, sLastY)

    const dX = worldX - lastPoint.x
    const dY = worldY - lastPoint.y

    let destX
    let destY

    if (Math.abs(dX / 2) > Math.abs(dY)) {
      destX = lastPoint.x + dX
      destY = lastPoint.y
    } else if (Math.abs(dY / 2) > Math.abs(dX)) {
      destX = lastPoint.x
      destY = lastPoint.y + dY
    } else {
      if (Math.abs(dX) > Math.abs(dY)) {
        destX = lastPoint.x + dX
        destY = lastPoint.y + Math.sign(dY) * Math.abs(dX)
      } else if (Math.abs(dY) > Math.abs(dX)) {
        destX = lastPoint.x + Math.sign(dX) * Math.abs(dY)
        destY = lastPoint.y + dY
      } else {
        destX = lastPoint.x + dX
        destY = lastPoint.y + dY
      }
    }

    const [sDestX, sDestY] = worldToScreen(destX, destY)
    ctx.lineTo(sDestX, sDestY)
    ctx.strokeStyle = 'red'
    ctx.lineWidth = scale.value / 2
    ctx.stroke()

    cursor.value = [destX, destY]
  }

  ctx.beginPath()
  const [sCursorX, sCursorY] = worldToScreen(cursor.value[0], cursor.value[1])
  ctx.arc(sCursorX, sCursorY, 8, 0, Math.PI * 2)
  ctx.fillStyle = 'black'
  ctx.strokeStyle = 'black'
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
  data.push_point(0, Point.new(cursor.value[0], cursor.value[1]))
  svg.value = data.svg()
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
