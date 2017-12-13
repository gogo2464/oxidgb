pipeline {
  agent any
  stages {
    stage('Linux') {
      steps {
        sh 'cd glutin_frontend && cargo build --release'
      }
    }
    stage('Deploy') {
      steps {
        archiveArtifacts 'target/release/oxidgb_glutin'
      }
    }
  }
}