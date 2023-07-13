<template>
  <div>
    <a-modal v-model:visible="visible" title="Basic Modal">
      <a-form
        :model="formState"
        name="basic"
        :label-col="{ span: 8 }"
        :wrapper-col="{ span: 16 }"
        autocomplete="off"
        @finish="onFinish"
        @finishFailed="onFinishFailed"
      >
        <a-form-item label="阅读方向" name="readDirection">
          <a-select v-model:value="formState.username">
            <a-select-option value="1">从上到下</a-select-option>
            <a-select-option value="2">从左到右</a-select-option>
          </a-select>
        </a-form-item>
      </a-form>
    </a-modal>
  </div>
</template>
<script setup lang="ts">
import {computed, reactive, ref, watch} from 'vue';

  interface FormState {
    username: string;
    password: string;
    remember: boolean;
  }

  const props = defineProps<{ show: boolean }>();
  console.log("model props: ", props);
  const emit = defineEmits<{
    (e: 'update:show', visible: boolean): void;
  }>();

  let visible = ref(props.show);
  watch(() => props.show, (newVal) => {
    console.log('show: ', newVal);
    visible.value = newVal
  })
  watch(visible, () => {
    emit('update:show', visible.value);
  });

  function showModal() {
    visible.value = true;
  }

  const formState = reactive<FormState>({
    username: '',
    password: '',
    remember: true,
  });
  const onFinish = (values: any) => {
    console.log('Success:', values);
  };

  const onFinishFailed = (errorInfo: any) => {
    console.log('Failed:', errorInfo);
  };
</script>
