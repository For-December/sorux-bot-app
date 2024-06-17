<script setup lang="ts">
import {onMounted} from 'vue'
import {useUserStore} from "@/store/userStore.ts";
import {useRouter} from "vue-router";
import {qrcodeImageSrc, startQrcodeListener} from "@/utils/globals.ts";

const userStore = useUserStore();

// const router = useRouter()

const onLogin = () => {
  userStore.login()
  // router.push("/")
}




// const imageSrc = ref('');



onMounted(()=>{
  startQrcodeListener()
})
</script>

<template>
  <h1 class="text-[#adacda]">1211</h1>
  <div class="flex-center">
    <el-card class="w-[40vw]">
      <template #header>
        <h1 class="flex-center text-[#adacda]">扫码登录</h1>
      </template>

      <div class="p-xy">
        <div v-if="qrcodeImageSrc" class="flex-center">
          <el-image :src="qrcodeImageSrc" class="w-[20vw]">
            <template #placeholder>
              1111
            </template>
          </el-image>
        </div>
        <div v-else class="flex-center">

          <div class="image-slot">
            正在获取登录二维码，请稍后
            <span class="dot">...</span>
          </div>

        </div>
      </div>


    </el-card>
  </div>


  <el-button type="success" @click="onLogin">
    LOGIN
  </el-button>

</template>

<style lang="scss" scoped>
.image-slot {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 10vw;
  background: var(--el-fill-color-light);
  color: var(--el-text-color-secondary);
  font-size: 1vw;
}

.dot {
  animation: dot 2s infinite steps(3, start);
  overflow: hidden;
}
</style>