<script setup lang="ts">
import { onMounted, ref } from "vue";
import Modal from "bootstrap/js/dist/modal";
import type { Instructions } from "./AssemblyEditor.vue";

const emit = defineEmits<{ (e: "import", instructions: Instructions): void }>();

const input = ref("");
const modalRef = ref<HTMLDivElement>();
const modal = ref<Modal>();

onMounted(() => {
  modal.value = new Modal(modalRef.value!);
});

function importInstructions() {
  emit("import", JSON.parse(input.value));
  modal.value!.hide();
  input.value = "";
}
</script>

<template>
  <button type="button" class="btn btn-outline-primary" @click="modal!.show">
    Import
  </button>

  <div
    class="modal fade"
    tabindex="-1"
    aria-labelledby="importModalLabel"
    aria-hidden="true"
    ref="modalRef"
  >
    <div class="modal-dialog">
      <div class="modal-content">
        <div class="modal-header">
          <h1 class="modal-title fs-5" id="importModalLabel">
            Import from JSON
          </h1>
          <button
            type="button"
            class="btn-close"
            data-bs-dismiss="modal"
            aria-label="Close"
          />
        </div>
        <div class="modal-body">
          <textarea class="form-control" v-model="input" />
        </div>
        <div class="modal-footer">
          <button
            type="button"
            class="btn btn-secondary"
            data-bs-dismiss="modal"
          >
            Close
          </button>
          <button
            type="button"
            class="btn btn-primary"
            @click="importInstructions"
          >
            Import
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
