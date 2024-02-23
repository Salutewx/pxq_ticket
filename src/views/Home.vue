<script setup lang="ts">
import { invoke, shell } from "@tauri-apps/api"
import { onMounted } from "vue";
import { RouterLink, RouterView } from "vue-router";
import { UserProfileResult } from '../types/user'
import { ref } from 'vue'
import { toast } from "vue3-toastify";
import router from "../router";
import { set_value } from "../store";

const avatarSrc = ref("")

const onClickGithub = () => {
  shell.open('https://github.com/ClassmateLin/pxq_ticket')
}

onMounted(async ()=>{
  console.log("hello")
  try {
    const res: UserProfileResult = await invoke("get_user_profile");
    console.log(res)
    if (res.statusCode == 200) {
      avatarSrc.value = res.data.avatar
      toast.success(`用户:${res.data.nickname}登录成功`)
      return
    }{
      toast.error(`获取用户信息失败, ${res.comments}`)
      router.push("login")
    }
  } catch (error: any) {
    toast.error(error)
  }
})


const logout = async () => {
  await set_value("access_token", null)
  await set_value("refresh_token", null)
  toast.success("成功退出账号")
  router.push('login')
}

</script>

<template>
  <div class="navbar bg-base-100">
    <div class="navbar-start">
      <a class="btn btn-ghost text-xl avatar c" @click="onClickGithub">Github</a>
    </div>
    <div class="navbar-center hidden lg:flex">
      <ul class="menu menu-horizontal px-1">
        <li>
          <RouterLink to="/task">任务</RouterLink>
        </li>
        <li>
          <RouterLink to="/order">订单</RouterLink>
        </li>
      </ul>
    </div>
    <div class="navbar-end">
      <div class="dropdown dropdown-end">
        <div tabindex="0" role="button" class="btn btn-ghost btn-circle avatar">
          <div class="w-10 rounded-full">
            <img alt="Tailwind CSS Navbar component" :src="avatarSrc" />
          </div>
        </div>
        <ul tabindex="0" class="menu menu-sm dropdown-content mt-3 z-[1] p-2 shadow bg-base-100 rounded-box w-52">
          <li @click="logout"><a>退出</a></li>
        </ul>
      </div>
    </div>
  </div>

  <div class="flex">
    <router-view v-slot="{ Component }">
      <keep-alive>
        <component :is="Component" :key="$route.name" v-if="$route.meta.keepAlive" />
      </keep-alive>
      <component :is="Component" :key="$route.name" v-if="!$route.meta.keepAlive" />
    </router-view>
  </div></template>
<style scoped></style>
