import {
  Modal,
  ModalBody,
  ModalContent,
  ModalHeader,
} from "@nextui-org/react";

export interface UnimplementedModalProps {
  isOpen: boolean,
  onOpenChange: () => void,
}

export function UnimplementedModal(props: UnimplementedModalProps) {
  return (
    <Modal isOpen={props.isOpen} onOpenChange={props.onOpenChange}>
      <ModalContent>
        {() => (
          <>
            <ModalHeader className="flex flex-col">未実装</ModalHeader>
            <ModalBody>
              <p>未実装の機能です。しばらく待ってね❤️</p>
            </ModalBody>
          </>
        )}
      </ModalContent>
    </Modal>
  )
}
