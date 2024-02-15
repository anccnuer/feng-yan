<script setup>
import { ref } from 'vue'
import init,{encode_str,decode_str} from 'hello'
import { onMounted } from 'vue';
onMounted(()=>{
  init()
})
let msg
let msg2 = ref('')
let regex = /^疯言.*！$/;
function processing(){
  if(msg == '') return
  if(regex.test(msg)){
    console.log(msg);
    let temp;
    temp = msg.replace(/疯言：/g, "");
    temp = temp.replace(/，给爷死！/g, "")
    msg2.value = decode_str(temp)
    console.log(typeof(decode_str(temp)))
  }
  else {
    msg2.value = '疯言：'+encode_str(msg)+'，给爷死！';
  }
  
  console.log(msg2.value);
}
</script>

<template>
  <div>要加密/解密的文字</div>
  <textarea name="" id="" cols="30" rows="10" v-model="msg"></textarea>
  <div class="card">
    <button type="button" @click='processing'>加密/解密</button>
  </div>
  <div>加密/解密后的文字</div>
  <textarea name="" id="" cols="30" rows="10" v-model="msg2"></textarea>
</template>

