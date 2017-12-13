pipeline {
  agent none
  stages {
    stage('Build') {
      parallel {
        stage('Linux') {
          steps {
            sh 'cd glutin_frontend && cargo build --release'
          }
        }
        stage('Windows') {
          steps {
            sh 'cd glutin_frontend && cargo build --release --target=x86_64-pc-windows-gnu '
          }
        }
      }
    }
    stage('Deploy') {
      steps {
        archiveArtifacts 'target/release/oxidgb_glutin'
        archiveArtifacts 'target/**/release/oxidgb_glutin.exe'
      }
    }
  }
}