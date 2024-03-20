import { graphql } from '~/gql';

export const GetUserDetailQuery = graphql(`
  query GetUserDetail($id: ID!) {
    user(id: $id) {
      id
      createdAt
      updatedAt
      name
    }
    groups(id: $id) {
      id
      createdAt
      title
    }
  }
`);

export const CreateUserMutation = graphql(`
  mutation CreateUser($input: CreateUserInput!) {
    createUser(input: $input) {
      id
    }
  }
`);

export const GetGroupDetailQuery = graphql(`
  query GetGroupDetail($id: ID!) {
    group(id: $id) {
      id
      createdAt
      title
      participants {
        id
        name
      }
      payments {
        id
        createdAt
        title
        creditors {
          user {
            id
            name
          }
          amount
        }
      }
      warikan {
        from {
          id
          name
        }
        to {
          id
          name
        }
        amount
      }
    }
  }
`);

export const CreateGroupMutation = graphql(`
  mutation CreateGroup($input: CreateGroupInput!) {
    createGroup(input: $input) {
      id
    }
  }
`);

export const GetPaymentDetailQuery = graphql(`
  query GetPaymentDetail($id: ID!) {
    payment(id: $id) {
      id
      createdAt
      title
      creditors {
        user {
          id
          name
        }
        amount
      }
      debtors {
        user {
          id
          name
        }
        amount
      }
    }
  }
`)

export const CreatePaymentMutation = graphql(`
  mutation CreatePayment($input: CreatePaymentInput!) {
    createPayment(input: $input) {
      id
    }
  }
`)

export const UpdatePaymentMutation = graphql(`
  mutation UpdatePayment($input: UpdatePaymentInput!) {
    updatePayment(input: $input) {
      id
    }
  }
`)
