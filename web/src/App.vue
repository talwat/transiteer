<script setup lang="ts">
import { computed, ref, useTemplateRef } from 'vue'
import { map_data as mapData } from '/wasm'

let data = mapData()
let viewBox = ref([0, 0, 256, 256])
let viewBoxString = computed(() => viewBox.value.join(' '))

let held = false
let offset = [0, 0]
let initial = [0, 0]

let design = useTemplateRef('design')

function move(event: MouseEvent) {
  if (!held || design.value == null) return

  const scale = Math.max(256 / window.innerWidth, 256 / window.innerHeight)

  viewBox.value[0] = initial[0] + (offset[0] - event.clientX) * scale
  viewBox.value[1] = initial[1] + (offset[1] - event.clientY) * scale
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
</script>

<template>
  <div id="view">
    <svg
      ref="design"
      id="design"
      xmlns="http://www.w3.org/2000/svg"
      :viewBox="viewBoxString"
      v-html="data"
      @mousemove="move"
      @mousedown="down"
      @mouseup="up"
      oncontextmenu="return false"
    ></svg>
    <div id="grid"></div>
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
}

#design {
  top: 0;
  margin: 0;
  position: absolute;
  overflow: hidden;
  width: 100%;
  height: 100%;
  -webkit-user-select: none;
  -moz-user-select: none;
  user-select: none;
}
</style>
