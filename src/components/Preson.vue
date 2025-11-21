
<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <button @click="increment">点击次数: {{ count }}</button>
    <p>计算属性: {{ doubledCount }}</p>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'

// 响应式数据
const msg = ref('欢迎使用 Vue 3!')
const count = ref(0)

// 计算属性
const doubledCount = computed(() => count.value * 2)

// 方法
const increment = () => {
  count.value++
}

// 生命周期钩子
onMounted(() => {
  console.log('组件已挂载')
})
</script>

<style scoped>
.hello {
  text-align: center;
  padding: 20px;
}
button {
  margin: 10px;
  padding: 8px 16px;
  background-color: #42b983;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}
button:hover {
  background-color: #3aa876;
}
</style>

