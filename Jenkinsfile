pipeline {
  agent {
    kubernetes {
      yamlFile 'KubernetesBuilder.yaml'
    }
  }
  stages {
    stage('Build') {
      steps {
        checkout scm
        container('rust') {
          sh 'cargo build --release'
        }
      }
    }
    stage('Test') {
      steps {
        checkout scm
        container('rust') {
          withCredentials([string(credentialsId: 'coveralls_search_and_sip', variable: 'coveralls_search_and_sip')]) {
            sh 'cargo test'
            sh 'cargo tarpaulin --coveralls ${coveralls_search_and_sip}'
          }
        }
      }
    }
    stage('Copy Artifacts') {
      steps {
        container('rust') {
          sh 'cp target/release/search_and_sip /workspace/opt/app/shared/search_and_sip'
          sh 'cp Dockerfile /workspace/opt/app/shared/Dockerfile'
        }
      }
    }
    stage('Release') {
      steps {
        container('kaniko') {
          sh 'cp /workspace/opt/app/shared/search_and_sip  /workspace'
          sh 'cp /workspace/opt/app/shared/Dockerfile /workspace'
          sh 'ulimit -n 10000'
          sh '/kaniko/executor -f Dockerfile --destination=docker.ultimaengineering.io/search_and_sip:${BRANCH_NAME}-${BUILD_NUMBER} --destination=docker.ultimaengineering.io/search_and_sip:latest'
        }
      }
    }
  }
}
